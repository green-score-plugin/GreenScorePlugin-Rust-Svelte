use sqlx::MySqlPool;
use backend::service::monitored_website_service::MonitoredWebsiteService;
use backend::models::monitored_website::MonitoredWebsite;
use backend::dto::lpc_dto::LastPageConsultedInfos;
use backend::repository::monitored_website_repository::MonitoredWebsiteRepository;

#[sqlx::test]
async fn devrait_sauvegarder_et_mettre_a_jour_total_monitored_website_data(pool: MySqlPool) {
    // GIVEN
    let website = MonitoredWebsite {
        id: 0,
        user_id: 1, // L'utilisateur 1 a 100.0 d'empreinte d'après les migrations
        url_domain: "example.com".to_string(),
        url_full: "https://example.com/test".to_string(),
        queries_quantity: 10,
        carbon_footprint: 5.5,
        data_transferred: 1024,
        resources: 5,
        loading_time: 1.5,
        country: "FR".to_string(),
    };

    // WHEN
    let result = MonitoredWebsiteService::save_monitored_website_data(&pool, &website).await;

    // THEN
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), 105.5);
}

#[sqlx::test]
async fn devrait_retourner_lpc_complet_avec_parametres_fournis(pool: MySqlPool) {
    // GIVEN
    let params = LastPageConsultedInfos {
        url_full: "https://example.com/page".to_string(),
        queries_quantity: 15,
        carbon_footprint: 1000.0,
        data_transferred: 500.0,
        loading_time: 0.5,
        country: "FR".to_string(),
    };

    // WHEN
    let response = MonitoredWebsiteService::lpc(&pool, Some(1), Some(params)).await.unwrap();

    // THEN
    assert!(response.success);
    assert_eq!(response.lpc_infos.unwrap().url_full, "https://example.com/page");
    assert!(response.letter.is_some());
    assert!(response.equivalents.is_some());
}

#[sqlx::test]
async fn devrait_retourner_lpc_sans_parametres_via_dernier_historique(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "last-site.com".to_string(), url_full: "https://last-site.com/home".to_string(),
        queries_quantity: 5, carbon_footprint: 1000.0, data_transferred: 300, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let response = MonitoredWebsiteService::lpc(&pool, Some(1), None).await.unwrap();

    // THEN
    assert!(response.success);
    assert_eq!(response.lpc_infos.unwrap().url_full, "https://last-site.com/home");
    assert!(response.letter.is_some());
}

#[sqlx::test]
async fn devrait_retourner_lpc_vide_si_utilisateur_aucun_historique(pool: MySqlPool) {
    // WHEN
    let response = MonitoredWebsiteService::lpc(&pool, Some(2), None).await.unwrap();

    // THEN
    assert!(response.success);
    assert!(response.lpc_infos.is_none());
    assert!(response.letter.is_none());
}

#[sqlx::test]
async fn devrait_calculer_get_my_average_daily_carbon_footprint(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "a.com".to_string(), url_full: "a.com".to_string(),
        queries_quantity: 1, carbon_footprint: 10.0, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    let w2 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "b.com".to_string(), url_full: "b.com".to_string(),
        queries_quantity: 1, carbon_footprint: 20.0, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w2).await.unwrap();

    // WHEN
    let avg = MonitoredWebsiteService::get_my_average_daily_carbon_footprint(&pool, 1).await;

    // THEN
    assert_eq!(avg.unwrap(), 15.0);
}

#[sqlx::test]
async fn devrait_calculer_get_average_daily_carbon_footprint_global(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "x.com".to_string(), url_full: "x.com".to_string(),
        queries_quantity: 1, carbon_footprint: 10.0, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    let w2 = MonitoredWebsite {
        id: 0, user_id: 2, url_domain: "y.com".to_string(), url_full: "y.com".to_string(),
        queries_quantity: 1, carbon_footprint: 5.0, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w2).await.unwrap();

    // WHEN
    let avg = MonitoredWebsiteService::get_average_daily_carbon_footprint(&pool).await;

    // THEN
    assert_eq!(avg.unwrap(), 7.5);
}

#[sqlx::test]
async fn devrait_recuperer_le_total_consumption_by_user(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "a.com".to_string(), url_full: "a.com".to_string(),
        queries_quantity: 1, carbon_footprint: 2.2, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let total = MonitoredWebsiteService::get_total_consumption_by_user(&pool, 1).await;

    // THEN
    assert_eq!(total.unwrap(), 2.2);
}

#[sqlx::test]
async fn devrait_recuperer_le_top_5_polluting_sites(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "a.com".to_string(), url_full: "a.com".to_string(),
        queries_quantity: 1, carbon_footprint: 2.227, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let top_sites = MonitoredWebsiteService::get_top5_polluting_sites_by_user(&pool, 1).await.unwrap();

    // THEN
    assert!(!top_sites.is_empty());
    // On vérifie que la valeur de retour est bien arrondie à deux décimales (2.227 -> 2.23)
    let site = top_sites.into_iter().find(|s| s.url_domain == "a.com");
    assert!(site.is_some());
    assert_eq!(site.unwrap().total_footprint, 2.23);
}

#[sqlx::test]
async fn devrait_recuperer_la_consommation_hebdomadaire_et_arrondir(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "w.com".to_string(), url_full: "w.com".to_string(),
        queries_quantity: 1, carbon_footprint: 3.333, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let results = MonitoredWebsiteService::get_weekly_consumption_by_user(&pool, 1).await.unwrap();

    // THEN
    assert!(!results.is_empty());
    assert_eq!(results[0].value, 3.33); // 3.333 arrondi à 3.33
    // Vérifier que le formatage "S{week}" s'applique bien
    assert!(results[0].label.starts_with('S'));
}

#[sqlx::test]
async fn devrait_recuperer_la_consommation_mensuelle_et_arrondir(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "m.com".to_string(), url_full: "m.com".to_string(),
        queries_quantity: 1, carbon_footprint: 4.445, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let results = MonitoredWebsiteService::get_monthly_consumption_by_user(&pool, 1).await.unwrap();

    // THEN
    assert!(!results.is_empty());
    assert_eq!(results[0].value, 4.45); // 4.445 arrondi à 4.45
}

#[sqlx::test]
async fn devrait_recuperer_la_consommation_journaliere_et_arrondir(pool: MySqlPool) {
    // GIVEN
    let w1 = MonitoredWebsite {
        id: 0, user_id: 1, url_domain: "d.com".to_string(), url_full: "d.com".to_string(),
        queries_quantity: 1, carbon_footprint: 5.556, data_transferred: 1, resources: 1, loading_time: 1.0, country: "FR".into()
    };
    MonitoredWebsiteRepository::save_monitored_website_data(&pool, &w1).await.unwrap();

    // WHEN
    let results = MonitoredWebsiteService::get_daily_consumption_by_user(&pool, 1).await.unwrap();

    // THEN
    assert!(!results.is_empty());
    assert_eq!(results[0].value, 5.56); // 5.556 arrondi à 5.56
}
