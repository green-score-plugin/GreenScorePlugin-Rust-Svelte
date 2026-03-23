use backend::controllers::lpc_controller;
use backend::dto::lpc_dto::{LastPageConsultedInfos, LastPageConsultedResponse};
use backend::dto::user_full::UserFull;
use backend::models::user::User;
use backend::middleware::auth::AuthenticatedUser;
use axum::extract::{Query, State};
use sqlx::{MySqlPool, Row};

fn create_dummy_user_full(id: i64) -> UserFull {
    UserFull {
        user: User {
            id,
            id_organisation: None,
            id_service: None,
            email: "test@example.com".to_string(),
            prenom: "Test".to_string(),
            nom: "User".to_string(),
            est_admin: false,
            total_carbon_footprint: 0.0,
        },
        organisation: None,
        service: None,
    }
}

#[sqlx::test]
async fn test_lpc_controller_integration(pool: MySqlPool) {
    // Run migrations from the migrations folder
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    // Seed some data for advice and equivalents so we get a rich response
    sqlx::query("INSERT INTO advice (advice, is_dev, title, icon) VALUES ('Dev advice', 1, '', '')").execute(&pool).await.ok();
    sqlx::query("INSERT INTO advice (advice, is_dev, title, icon) VALUES ('User advice', 0, '', '')").execute(&pool).await.ok();
    sqlx::query("INSERT INTO equivalent (name, equivalent, icon_thumbnail) VALUES ('Car', 0.5, 'car.png')").execute(&pool).await.ok();

    let authenticated_user = AuthenticatedUser(create_dummy_user_full(1));

    let params = LastPageConsultedInfos {
        link: "https://example.com/page".to_string(),
        queries_quantity: 10,
        carbon_footprint: 0.1, // Grade A
        data_transferred: 100.0,
        loading_time: 1.2,
        country: "FR".to_string(),
    };

    let result = lpc_controller::lpc(
        State(pool.clone()),
        authenticated_user,
        Query(params.clone()),
    ).await;

    assert!(result.is_ok());

    let response = result.unwrap().0;

    // Check general success
    assert!(response.success);

    // Check if the input info was returned
    assert!(response.lpc_infos.is_some());
    let returned_info = response.lpc_infos.unwrap();
    assert_eq!(returned_info.link, params.link);
    assert_eq!(returned_info.carbon_footprint, params.carbon_footprint);

    // Check computed fields
    assert_eq!(response.letter, Some("A".to_string())); // 0.1 is A
    assert!(response.env_nomination.is_some());

    // Check side effects: Data should be inserted into monitored_websites
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monitored_websites WHERE user_id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "Should have inserted one record into monitored_websites");
}
