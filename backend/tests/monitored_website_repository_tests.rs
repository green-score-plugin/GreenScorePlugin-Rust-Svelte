use backend::repository::monitored_website_repository::MonitoredWebsiteRepository;
use backend::models::monitored_website::MonitoredWebsite;
use sqlx::MySqlPool;

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