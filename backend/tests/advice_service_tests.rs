use backend::service::advice_service::AdviceService;
use sqlx::MySqlPool;

#[sqlx::test]
async fn devrait_retourner_40_advices_via_service(pool: MySqlPool) {
    // GIVEN
    // Les données proviennent des migrations par défaut

    // WHEN
    let advices = AdviceService::get_all_advice(&pool).await.expect("Failed to get advices via service");

    // THEN
    assert_eq!(advices.len(), 40, "On devrait avoir exactement 3 conseils (ceux de la migration) en passant par le service");
}

#[sqlx::test]
async fn devrait_retourner_luminosite_via_service_non_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = false;
    let expected_keys = vec![
        "data.advice.adjust_brightness", "data.advice.close_unused_tabs", "data.advice.block_ads",
        "data.advice.eco_search_engines", "data.advice.disable_autoplay", "data.advice.download_necessary",
        "data.advice.clear_cache", "data.advice.limit_extensions", "data.advice.data_saver_mode",
        "data.advice.mobile_optimized_sites", "data.advice.low_res_video", "data.advice.logout_unused",
        "data.advice.lightweight_browser", "data.advice.wifi_over_data", "data.advice.avoid_heavy_ads",
        "data.advice.plan_searches", "data.advice.text_mode", "data.advice.share_links",
        "data.advice.disable_push", "data.advice.close_background_apps"
    ];

    // WHEN
    let advice_text = AdviceService::get_one_random_advice(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice via service");

    // THEN
    assert!(
        expected_keys.contains(&advice_text.as_str()),
        "Le conseil retourné ({}) devrait être l'un des conseils non-dev définis dans la migration", advice_text
    );
}

#[sqlx::test]
async fn devrait_retourner_requetes_ou_fichier_via_service_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = true;
    let expected_keys = vec![
        "data.advice.optimize_sql", "data.advice.minify_files", "data.advice.server_cache",
        "data.advice.reduce_http_requests", "data.advice.use_webp", "data.advice.lazy_loading",
        "data.advice.avoid_loops", "data.advice.eco_servers", "data.advice.use_cdn",
        "data.advice.reduce_resource_usage", "data.advice.efficient_algorithms", "data.advice.reduce_cookies",
        "data.advice.http_caching", "data.advice.lightweight_frameworks", "data.advice.adapted_db",
        "data.advice.test_performance", "data.advice.cloud_solutions", "data.advice.disable_logs",
        "data.advice.offline_reports", "data.advice.resource_audit"
    ];

    // WHEN
    let advice_text = AdviceService::get_one_random_advice(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice via service");

    // THEN
    assert!(
        expected_keys.contains(&advice_text.as_str()),
        "Le conseil retourné ({}) devrait être l'un des conseils dev définis dans la migration", advice_text
    );
}
