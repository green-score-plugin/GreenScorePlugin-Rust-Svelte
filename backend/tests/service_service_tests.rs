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
async fn devrait_retourner_erreur_si_service_existe_deja(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let nom_service = "Service Existant";

    // WHEN
    ServiceService::create_service(&pool, org_id, nom_service).await.unwrap();
    let result = ServiceService::create_service(&pool, org_id, nom_service).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.service_exists");
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
async fn devrait_retourner_erreur_si_assignation_service_inexistant(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let user_id = 1;

    // WHEN
    let result = ServiceService::assign_user_to_service(&pool, user_id, 99999, org_id).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.service_not_found");
}

#[sqlx::test]
async fn devrait_retourner_erreur_si_assignation_service_autre_organisation(pool: MySqlPool) {
    // GIVEN
    let org1 = 1;
    let org2 = 2; // Suppose qu'il y a une organisation 2 ou on utilisera un id bidon
    let user_id = 1;
    let services = ServiceService::create_service(&pool, org1, "Service Org 1").await.unwrap();
    let service_id = services.into_iter().find(|s| s.nom == "Service Org 1").unwrap().id;

    // WHEN
    let result = ServiceService::assign_user_to_service(&pool, user_id, service_id, org2).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.service_not_in_organisation");
}

#[sqlx::test]
async fn devrait_retourner_erreur_si_assignation_utilisateur_autre_organisation(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let user_id = 99999; // Utilisateur inexistant
    let services = ServiceService::create_service(&pool, org_id, "Service User Inexistant").await.unwrap();
    let service_id = services.into_iter().find(|s| s.nom == "Service User Inexistant").unwrap().id;

    // WHEN
    let result = ServiceService::assign_user_to_service(&pool, user_id, service_id, org_id).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.user_not_in_organisation");
}

#[sqlx::test]
async fn devrait_retourner_erreur_si_desassignation_utilisateur_autre_organisation(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;
    let user_id = 99999;

    // WHEN
    let result = ServiceService::unassign_user_from_service(&pool, user_id, org_id).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.user_not_in_organisation");
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

#[sqlx::test]
async fn devrait_retourner_erreur_si_suppression_service_inexistant(pool: MySqlPool) {
    // GIVEN
    let org_id = 1;

    // WHEN
    let result = ServiceService::delete_service(&pool, 99999, org_id).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.service_not_found");
}

#[sqlx::test]
async fn devrait_retourner_erreur_si_suppression_service_autre_organisation(pool: MySqlPool) {
    // GIVEN
    let org1 = 1;
    let org2 = 2; // Simulation d'une tentative de suppression depuis une autre org
    let services = ServiceService::create_service(&pool, org1, "Service Org 1 a ne pas suppr").await.unwrap();
    let service_id = services.into_iter().find(|s| s.nom == "Service Org 1 a ne pas suppr").unwrap().id;

    // WHEN
    let result = ServiceService::delete_service(&pool, service_id, org2).await;

    // THEN
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "errors.service_not_in_organisation");
}

#[sqlx::test]
async fn devrait_retourner_db_error_si_erreur_bdd(pool: MySqlPool) {
    // GIVEN - Fermer le pool pour forcer une erreur de base de données (PoolClosed)
    pool.close().await;

    // WHEN
    let res_create = ServiceService::create_service(&pool, 1, "Test Db Error").await;
    let res_get_org = ServiceService::get_organisation_services(&pool, 1).await;
    let res_get_user = ServiceService::get_services_by_user_id(&pool, 1).await;
    let res_delete = ServiceService::delete_service(&pool, 1, 1).await;
    let res_assign = ServiceService::assign_user_to_service(&pool, 1, 1, 1).await;
    let res_unassign = ServiceService::unassign_user_from_service(&pool, 1, 1).await;

    // THEN
    assert!(res_create.is_err());
    assert!(res_create.unwrap_err().starts_with("db_error:"));

    assert!(res_get_org.is_err());
    assert!(res_get_org.unwrap_err().starts_with("db_error:"));

    assert!(res_get_user.is_err());
    assert!(res_get_user.unwrap_err().starts_with("db_error:"));

    assert!(res_delete.is_err());
    assert!(res_delete.unwrap_err().starts_with("db_error:"));

    assert!(res_assign.is_err());
    assert!(res_assign.unwrap_err().starts_with("db_error:"));

    assert!(res_unassign.is_err());
    assert!(res_unassign.unwrap_err().starts_with("db_error:"));
}
