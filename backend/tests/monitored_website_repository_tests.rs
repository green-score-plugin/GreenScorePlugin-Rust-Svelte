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
    assert_eq!(last_search.url_full, website.url_full);
    assert_eq!(last_search.queries_quantity, website.queries_quantity as i32);
    assert_eq!(last_search.data_transferred, website.data_transferred as f64);
    assert_eq!(last_search.country, website.country);
    assert_eq!(last_search.carbon_footprint, website.carbon_footprint as f64);
}