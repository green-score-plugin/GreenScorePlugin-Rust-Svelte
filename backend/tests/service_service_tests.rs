use sqlx::MySqlPool;
use backend::service::service_service::ServiceService;

#[sqlx::test]
async fn devrait_creer_un_service(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let nom_service = "Nouveau Service Test";

    // WHEN
    let services = ServiceService::create_service(&pool, org_id, nom_service).await.unwrap();

    // THEN
    assert!(services.iter().any(|s| s.nom == nom_service));
}

#[sqlx::test]
async fn devrait_retourner_erreur_si_nom_vide(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let nom_service = "   ";

    // WHEN
    let result = ServiceService::create_service(&pool, org_id, nom_service).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.validation_service_name_required");
}

#[sqlx::test]
async fn devrait_recuperer_services_organisation(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;

    // WHEN
    let services = ServiceService::get_organisation_services(&pool, org_id).await.unwrap();

    // THEN
    assert!(!services.is_empty());
}

#[sqlx::test]
async fn devrait_recuperer_services_utilisateur(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let user_id = 1; // Un utilisateur existant dans l'organisation 1

    let services_crees = ServiceService::create_service(&pool, org_id, "Service user test").await.unwrap();
    let service_id = services_crees.into_iter().find(|s| s.nom == "Service user test").unwrap().id;

    ServiceService::assign_user_to_service(&pool, user_id, service_id, org_id).await.unwrap();

    // WHEN
    let user_services = ServiceService::get_services_by_user_id(&pool, user_id).await.unwrap();

    // THEN
    assert!(user_services.iter().any(|s| s.id == service_id));
}

#[sqlx::test]
async fn devrait_assigner_et_desassigner_utilisateur(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let user_id = 1; // Utilisateur existant dans l'organisation 1
    let services = ServiceService::get_organisation_services(&pool, org_id).await.unwrap();
    let service_id = services.first().expect("Devrait avoir au moins un service").id;

    // WHEN - Assigner
    let assign_result = ServiceService::assign_user_to_service(&pool, user_id, service_id, org_id).await;

    // THEN
    assert!(assign_result.is_ok());

    // WHEN - Désassigner
    let unassign_result = ServiceService::unassign_user_from_service(&pool, user_id, org_id).await;

    // THEN
    assert!(unassign_result.is_ok());
}

#[sqlx::test]
async fn devrait_supprimer_service(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let services = ServiceService::create_service(&pool, org_id, "Service a supprimer").await.unwrap();
    let service_id = services.into_iter().find(|s| s.nom == "Service a supprimer").unwrap().id;

    // WHEN
    let delete_result = ServiceService::delete_service(&pool, service_id, org_id).await;

    // THEN
    assert!(delete_result.is_ok());

    let remaining_services = ServiceService::get_organisation_services(&pool, org_id).await.unwrap();
    assert!(!remaining_services.iter().any(|s| s.id == service_id));
}
