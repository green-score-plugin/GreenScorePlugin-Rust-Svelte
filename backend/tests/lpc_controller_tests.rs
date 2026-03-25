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
async fn devrait_retourner_succes_et_enregistrer_donnees_pour_lpc(pool: MySqlPool) {
    // GIVEN
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Echec de la migration de la base de données");

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

    // WHEN
    let result = lpc_controller::lpc(
        State(pool.clone()),
        authenticated_user,
        Query(params.clone()),
    ).await;

    // THEN
    assert!(result.is_ok(), "Le contrôleur devrait retourner un résultat Ok");

    let response = result.unwrap().0;

    assert!(response.success, "La réponse devrait indiquer un succès");

    assert!(response.lpc_infos.is_some(), "Les informations LPC devraient être présentes");
    let returned_info = response.lpc_infos.unwrap();
    assert_eq!(returned_info.link, params.link, "Le lien retourné devrait correspondre à l'entrée");
    assert_eq!(returned_info.carbon_footprint, params.carbon_footprint, "L'empreinte carbone retournée devrait correspondre");

    assert_eq!(response.letter, Some("A".to_string()), "Le grade devrait être A pour une empreinte de 0.1");
    assert!(response.env_nomination.is_some(), "Une nomination environnementale devrait être retournée");

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM monitored_websites WHERE user_id = ?")
        .bind(1)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);
    assert_eq!(count, 1, "Devrait avoir inséré un enregistrement dans monitored_websites");
}
