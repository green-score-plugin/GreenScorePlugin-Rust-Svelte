use backend::repository::monitored_website_repository::MonitoredWebsiteRepository;
use backend::models::monitored_website::MonitoredWebsite;
use sqlx::MySqlPool;

// Helper pour insérer rapidement des données de test
async fn insert_measure(pool: &MySqlPool, user_id: i64, footprint: f64, date_sql: &str) {
    let query = format!(
        "INSERT INTO monitored_website (user_id, carbon_footprint, creation_date, url_domain) VALUES (?, ?, {}, 'generated.com')",
        date_sql
    );
    sqlx::query(&query)
        .bind(user_id)
        .bind(footprint)
        .execute(pool)
        .await
        .unwrap();
}

#[sqlx::test]
async fn devrait_sauvegarder_save_monitored_website_data(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    let website: MonitoredWebsite = MonitoredWebsite{
        id: 1,
        url_domain: "test.example.com".to_string(),
        user_id,
        queries_quantity: 1,
        data_transferred: 2,
        resources: 3,
        loading_time: 4.0,
        carbon_footprint: 5.0,
        url_full: "https://test.example.com/test".to_string(),
        country: "France".to_string(),
    };

    // WHEN
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();
    let last_search = MonitoredWebsiteRepository::get_last_search_information_by_user(&pool, user_id).await.unwrap().unwrap();

    // THEN
    assert_eq!(last_search.url_full, website.url_full, "L'URL complète sauvegardée ne correspond pas.");
    assert_eq!(last_search.queries_quantity, website.queries_quantity as i32, "La quantité de requêtes sauvegardée ne correspond pas.");
    assert_eq!(last_search.data_transferred, website.data_transferred as f64, "Les données transférées sauvegardées ne correspondent pas.");
    assert_eq!(last_search.country, website.country, "Le pays sauvegardé ne correspond pas.");
    assert_eq!(last_search.carbon_footprint, website.carbon_footprint, "L'empreinte carbone sauvegardée ne correspond pas.");
}

#[sqlx::test]
async fn devrait_retourner_total_empreinte_carbone_get_total_consumption_by_user(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    for i in 1..=3 {
        let website = MonitoredWebsite {
            id: i,
            url_domain: "test.example.com".to_string(),
            user_id,
            queries_quantity: 10,
            data_transferred: 100,
            resources: 5,
            loading_time: 1.5,
            carbon_footprint: 10.0,
            url_full: format!("https://test.example.com/page/{}", i),
            country: "France".to_string(),
        };
        MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();
    }

    // WHEN
    let total_consumption = MonitoredWebsiteRepository::get_total_consumption_by_user(&pool, user_id).await.unwrap();

    // THEN
    assert_eq!(total_consumption, 30.0, "La consommation carbone totale calculée ({}) est incorrecte, attendu : 30.0", total_consumption);
}

#[sqlx::test]
async fn devrait_retourner_top5_polluting_sites_by_user(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    // Insérer 6 sites avec des empreintes croissantes
    for i in 1..=6 {
        let website = MonitoredWebsite {
            id: i,
            url_domain: format!("site{}.com", i),
            user_id,
            queries_quantity: 10,
            data_transferred: 100,
            resources: 5,
            loading_time: 1.5,
            carbon_footprint: (i as f64) * 10.0, // 10, 20, 30, 40, 50, 60
            url_full: format!("https://site{}.com", i),
            country: "France".to_string(),
        };
        MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();
    }

    // WHEN
    let top5 = MonitoredWebsiteRepository::get_top5_polluting_sites_by_user(&pool, user_id).await.unwrap();

    // THEN
    assert_eq!(top5.len(), 5, "Devrait retourner 5 sites");
    assert_eq!(top5[0].total_footprint, 60.0, "Le premier devrait être le plus polluant (60.0)");
    assert_eq!(top5[4].total_footprint, 20.0, "Le dernier devrait être le 5ème plus polluant (20.0)");
}

#[sqlx::test]
async fn devrait_retourner_top5_polluting_sites_by_organization(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    let org_id: i64 = 1;

    // Créer une organisation
    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Test Org', 'TEST')")
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();

    // Update user avec organisation
    sqlx::query("UPDATE user SET organisation_id = ? WHERE id = ?")
        .bind(org_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    // Insérer des sites pour l'user (qui est dans l'orga)
    for i in 1..=6 {
        let website = MonitoredWebsite {
            id: i,
            url_domain: format!("site{}.com", i),
            user_id,
            queries_quantity: 10,
            data_transferred: 100,
            resources: 5,
            loading_time: 1.5,
            carbon_footprint: (i as f64) * 10.0,
            url_full: format!("https://site{}.com", i),
            country: "France".to_string(),
        };
        MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();
    }

    // WHEN
    let top5 = MonitoredWebsiteRepository::get_top5_polluting_sites_by_organization(&pool, org_id).await.unwrap();

    // THEN
    assert_eq!(top5.len(), 5, "Devrait retourner 5 sites pour l'organisation");
    assert_eq!(top5[0].total_footprint, 60.0);
}

#[sqlx::test]
async fn devrait_retourner_total_organization_consumption(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    let org_id: i64 = 1;

    // Créer une organisation et assigner user
    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Test Org', 'TEST')")
        .bind(org_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("UPDATE user SET organisation_id = ? WHERE id = ?")
        .bind(org_id)
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    // Insérer 2 sites
    let website = MonitoredWebsite {
        id: 1,
        url_domain: "site1.com".to_string(),
        user_id,
        queries_quantity: 10, data_transferred: 100, resources: 5, loading_time: 1.5,
        carbon_footprint: 10.0,
        url_full: "https://site1.com".to_string(), country: "France".to_string(),
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();
    let website2 = MonitoredWebsite {
        id: 2,
        url_domain: "site2.com".to_string(),
        user_id,
        queries_quantity: 10, data_transferred: 100, resources: 5, loading_time: 1.5,
        carbon_footprint: 20.0,
        url_full: "https://site2.com".to_string(), country: "France".to_string(),
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website2).await.unwrap();

    // WHEN
    let total = MonitoredWebsiteRepository::total_organization_consumption(&pool, org_id).await.unwrap();

    // THEN
    assert_eq!(total, Some(30.0), "Total conso orga incorrect");
}

#[sqlx::test]
async fn devrait_retourner_get_daily_consumption_by_user(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    let website = MonitoredWebsite {
        id: 1,
        url_domain: "site1.com".to_string(),
        user_id,
        queries_quantity: 10, data_transferred: 100, resources: 5, loading_time: 1.5,
        carbon_footprint: 12.5,
        url_full: "https://site1.com".to_string(), country: "France".to_string(),
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();

    // WHEN
    let daily = MonitoredWebsiteRepository::get_daily_consumption_by_user(&pool, user_id).await.unwrap();

    // THEN
    // Comme l'insertion met la date du jour, on devrait avoir 1 entrée
    assert_eq!(daily.len(), 1, "Devrait avoir une entrée pour aujourd'hui");
    assert_eq!(daily[0].value, 12.5);
}

#[sqlx::test]
async fn devrait_retourner_average_daily_carbon_footprint_for_organization(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;
    let org_id: i64 = 1;

    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Test Org', 'TEST')")
        .bind(org_id).execute(&pool).await.unwrap();
    sqlx::query("UPDATE user SET organisation_id = ? WHERE id = ?")
        .bind(org_id).bind(user_id).execute(&pool).await.unwrap();

    let website = MonitoredWebsite {
        id: 1, url_domain: "site1.com".to_string(), user_id, queries_quantity: 10, data_transferred: 100, resources: 5, loading_time: 1.5,
        carbon_footprint: 100.0, url_full: "https://site1.com".to_string(), country: "France".to_string(),
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &website).await.unwrap();

    // WHEN
    let average = MonitoredWebsiteRepository::average_daily_carbon_footprint_for_organization(&pool, org_id).await;

    // THEN
    // 1 jour, total 100 -> moyenne 100
    assert_eq!(average, 100.0);
}

#[sqlx::test]
async fn devrait_retourner_get_weekly_consumption_by_user(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;

    // Insertion simplifiée via helper
    insert_measure(&pool, user_id, 10.0, "NOW()").await;
    insert_measure(&pool, user_id, 20.0, "DATE_SUB(NOW(), INTERVAL 2 WEEK)").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_weekly_consumption_by_user(&pool, user_id).await.unwrap();

    // THEN
    // On s'attend à 2 entrées (semaine courante et semaine -2)
    assert_eq!(result.len(), 2, "Devrait avoir 2 semaines de données");
    // L'ordre est ASC, donc la plus vieille (J-2 semaines) d'abord
    assert_eq!(result[0].2, 20.0, "La semaine passée a 20.0");
    assert_eq!(result[1].2, 10.0, "La semaine courante a 10.0");
}

#[sqlx::test]
async fn devrait_retourner_get_monthly_consumption_by_user(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;

    insert_measure(&pool, user_id, 5.0, "NOW()").await;
    insert_measure(&pool, user_id, 15.0, "DATE_SUB(NOW(), INTERVAL 2 MONTH)").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_monthly_consumption_by_user(&pool, user_id).await.unwrap();

    // THEN
    assert_eq!(result.len(), 2, "Devrait avoir 2 mois de données");
    assert_eq!(result[0].value, 15.0);
    assert_eq!(result[1].value, 5.0);
}

#[sqlx::test]
async fn devrait_retourner_get_my_average_daily_carbon_footprint(pool: MySqlPool) {
    // GIVEN
    let user_id: i64 = 1;

    insert_measure(&pool, user_id, 10.0, "NOW()").await;
    insert_measure(&pool, user_id, 20.0, "NOW()").await;
    insert_measure(&pool, user_id, 5.0, "DATE_SUB(NOW(), INTERVAL 1 DAY)").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_my_average_daily_carbon_footprint(&pool, user_id).await.unwrap();

    // THEN
    // On doit avoir 2 jours distincts
    // Attention l'ordre SQL n'est pas garanti par le repo car pas de ORDER BY dans la requête (GROUP BY day seulement)
    // Cependant MySQL retourne souvent dans l'ordre du GROUP BY
    let mut found_today = false;
    let mut found_yesterday = false;

    for (day, avg) in result {
        // day est une String "YYYY-MM-DD"
        if avg == 15.0 { found_today = true; }
        if avg == 5.0 { found_yesterday = true; }
    }

    assert!(found_today, "Devrait avoir une moyenne de 15.0 pour aujourd'hui");
    assert!(found_yesterday, "Devrait avoir une moyenne de 5.0 pour hier");
}

#[sqlx::test]
async fn devrait_retourner_get_average_daily_carbon_footprint_global(pool: MySqlPool) {
    // GIVEN
    let user1: i64 = 1; // Créé par la migration
    let user2: i64 = 2;

    // Création du user 2 manquant
    sqlx::query("INSERT INTO user (id, email, roles, password) VALUES (?, 'user2@test.com', '[]', 'pwd')")
        .bind(user2)
        .execute(&pool)
        .await
        .unwrap();

    insert_measure(&pool, user1, 10.0, "NOW()").await;
    insert_measure(&pool, user2, 20.0, "NOW()").await;

    // Moyenne globale pour aujourd'hui = (10+20)/2 = 15.0

    // WHEN
    let result = MonitoredWebsiteRepository::get_average_daily_carbon_footprint(&pool).await.unwrap();

    // THEN
    let today_avg = result.iter().find(|(_, avg)| *avg == 15.0);
    assert!(today_avg.is_some(), "Devrait trouver une moyenne globale de 15.0");
}

#[sqlx::test]
async fn devrait_retourner_get_daily_organization_consumption(pool: MySqlPool) {
    // GIVEN
    let org_id: i64 = 100;
    // Création orga
    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Org Daily', 'ORGCODE')")
        .bind(org_id).execute(&pool).await.unwrap();

    // 2 Users dans l'orga
    sqlx::query("INSERT INTO user (id, organisation_id, email, roles, password) VALUES (10, ?, 'u1@test.com', '[]', 'pwd')")
        .bind(org_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO user (id, organisation_id, email, roles, password) VALUES (11, ?, 'u2@test.com', '[]', 'pwd')")
        .bind(org_id).execute(&pool).await.unwrap();

    insert_measure(&pool, 10, 10.0, "NOW()").await;
    insert_measure(&pool, 11, 20.0, "NOW()").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_daily_organization_consumption(&pool, org_id).await.unwrap();

    // THEN
    assert_eq!(result.len(), 1, "Devrait avoir 1 jour de données");
    assert_eq!(result[0].value, 30.0, "Total journalier orga (10+20) devrait faire 30.0");
}

#[sqlx::test]
async fn devrait_retourner_get_weekly_organization_consumption(pool: MySqlPool) {
    // GIVEN
    let org_id: i64 = 101;
    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Org Weekly', 'ORGWEEK')")
        .bind(org_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO user (id, organisation_id, email, roles, password) VALUES (20, ?, 'u3@test.com', '[]', 'pwd')")
        .bind(org_id).execute(&pool).await.unwrap();

    insert_measure(&pool, 20, 100.0, "NOW()").await;
    insert_measure(&pool, 20, 50.0, "DATE_SUB(NOW(), INTERVAL 2 WEEK)").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_weekly_organization_consumption(&pool, org_id).await.unwrap();

    // THEN
    assert_eq!(result.len(), 2, "2 semaines de données attendues");
    // Tri ASC : la plus ancienne en premier
    assert_eq!(result[0].value, 50.0);
    assert_eq!(result[1].value, 100.0);
}

#[sqlx::test]
async fn devrait_retourner_get_monthly_organization_consumption(pool: MySqlPool) {
    // GIVEN
    let org_id: i64 = 102;
    sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, 'Org Monthly', 'ORGMONTH')")
        .bind(org_id).execute(&pool).await.unwrap();
    sqlx::query("INSERT INTO user (id, organisation_id, email, roles, password) VALUES (30, ?, 'u4@test.com', '[]', 'pwd')")
        .bind(org_id).execute(&pool).await.unwrap();

    insert_measure(&pool, 30, 200.0, "NOW()").await;
    insert_measure(&pool, 30, 100.0, "DATE_SUB(NOW(), INTERVAL 3 MONTH)").await;

    // WHEN
    let result = MonitoredWebsiteRepository::get_monthly_organization_consumption(&pool, org_id).await.unwrap();

    // THEN
    assert_eq!(result.len(), 2, "2 mois de données attendues");
    assert_eq!(result[0].value, 100.0);
    assert_eq!(result[1].value, 200.0);
}
