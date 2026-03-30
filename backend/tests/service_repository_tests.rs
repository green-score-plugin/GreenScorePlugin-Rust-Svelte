use sqlx::MySqlPool;
use backend::repository::service_repository::ServiceRepository;
use backend::repository::organisation_repository::OrganisationRepository;

#[sqlx::test]
async fn devrait_recuperer_un_service_par_son_id(pool: MySqlPool) {
    // GIVEN
    // 1. Créer une organisation pour lier le service
    let org_name = "Org For Service Test";
    let org_code = "SRV_TEST_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    // 2. Insérer un service manuellement
    let service_name = "Service Test";
    let res = sqlx::query(
        "INSERT INTO service (nom, id_organisation) VALUES (?, ?)"
    )
    .bind(service_name)
    .bind(org_id)
    .execute(&pool)
    .await
    .expect("Impossible d'insérer le service");

    let service_id = res.last_insert_id() as i64;

    // WHEN
    let found_service = ServiceRepository::find_by_id(&pool, service_id)
        .await
        .expect("Erreur lors de la récupération du service");

    // THEN
    assert!(found_service.is_some(), "Le service devrait être trouvé");
    let service = found_service.unwrap();
    assert_eq!(service.nom, service_name);
    assert_eq!(service.id_organisation, org_id);
}

#[sqlx::test]
async fn devrait_recuperer_un_service_par_son_nom_et_id_organisation(pool: MySqlPool) {
    // GIVEN
    let org_name = "Org For Service Find By Name";
    let org_code = "SRV_FIND_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    let service_name = "Service By Name";
    ServiceRepository::create_service(&pool, service_name, org_id)
        .await
        .expect("Impossible d'insérer le service");

    // WHEN
    let found_service = ServiceRepository::find_by_nom(&pool, service_name, org_id)
        .await
        .expect("Erreur lors de la récupération du service par nom");

    // THEN
    assert!(found_service.is_some(), "Le service devrait être trouvé par nom");
    let service = found_service.unwrap();
    assert_eq!(service.nom, service_name);
    assert_eq!(service.id_organisation, org_id);
}

#[sqlx::test]
async fn devrait_creer_un_service_et_retourner_son_id(pool: MySqlPool) {
    // GIVEN
    let org_name = "Org For Create Service";
    let org_code = "SRV_CREATE_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    let service_name = "Service Create Test";

    // WHEN
    let service_id = ServiceRepository::create_service(&pool, service_name, org_id)
        .await
        .expect("Impossible de créer le service");

    // THEN
    assert!(service_id > 0, "L'id du service devrait être > 0");
    let service_in_db = ServiceRepository::find_by_id(&pool, service_id)
        .await
        .expect("Erreur")
        .expect("Service introuvable");
    assert_eq!(service_in_db.nom, service_name);
}

#[sqlx::test]
async fn devrait_recuperer_tous_les_services_d_une_organisation(pool: MySqlPool) {
    // GIVEN
    let org_name = "Org List Service";
    let org_code = "SRV_LIST_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    ServiceRepository::create_service(&pool, "Service 1", org_id).await.unwrap();
    ServiceRepository::create_service(&pool, "Service 2", org_id).await.unwrap();

    // WHEN
    let services = ServiceRepository::get_services_by_organisation_id(&pool, org_id)
        .await
        .expect("Erreur lors de la récupération des services");

    // THEN
    assert_eq!(services.len(), 2, "L'organisation devrait avoir 2 services");
    let has_service_1 = services.iter().any(|s| s.nom == "Service 1");
    let has_service_2 = services.iter().any(|s| s.nom == "Service 2");
    assert!(has_service_1 && has_service_2, "Les bons services doivent être retournés");
}

#[sqlx::test]
async fn devrait_recuperer_les_services_associes_a_l_organisation_d_un_utilisateur(pool: MySqlPool) {
    // GIVEN
    let org_name = "Org User Service";
    let org_code = "SRV_USER_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    ServiceRepository::create_service(&pool, "User Service A", org_id).await.unwrap();
    ServiceRepository::create_service(&pool, "User Service B", org_id).await.unwrap();

    // Insert user mapped to this organization
    sqlx::query("INSERT INTO user (email, password, first_name, last_name, total_carbon_footprint) VALUES (?, ?, ?, ?, ?)")
        .bind("service_user_test@example.com")
        .bind("pwd")
        .bind("John")
        .bind("Doe")
        .bind(0.0)
        .execute(&pool)
        .await
        .unwrap();

    let user_id: i64 = sqlx::query_scalar("SELECT id FROM user WHERE email = ?")
        .bind("service_user_test@example.com")
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO organisation_user (user_id, organisation_id, est_admin) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(org_id)
        .bind(false)
        .execute(&pool)
        .await
        .unwrap();

    // WHEN
    let user_services = ServiceRepository::find_by_user_id(&pool, user_id)
        .await
        .expect("Erreur lors de la récupération des services via l'utilisateur");

    // THEN
    assert_eq!(user_services.len(), 2, "Devrait retourner 2 services (ceux de l'orga du user)");
}

#[sqlx::test]
async fn devrait_supprimer_un_service_par_son_id_et_dissocier_les_utilisateurs(pool: MySqlPool) {
    // GIVEN
    let org_name = "Org Delete Service";
    let org_code = "SRV_DEL_01";
    let org_id = OrganisationRepository::insert_organisation(&pool, org_name, org_code, None)
        .await
        .expect("Impossible d'insérer l'organisation");

    let service_id = ServiceRepository::create_service(&pool, "Service To Delete", org_id)
        .await
        .expect("Impossible de créer le service");

    // Insert user mapped to this service
    sqlx::query("INSERT INTO user (email, password, first_name, last_name, service_id, total_carbon_footprint) VALUES (?, ?, ?, ?, ?, ?)")
        .bind("user_with_service@example.com")
        .bind("pwd")
        .bind("Jane")
        .bind("Doe")
        .bind(service_id)
        .bind(0.0)
        .execute(&pool)
        .await
        .unwrap();

    let user_id: i64 = sqlx::query_scalar("SELECT id FROM user WHERE email = ?")
        .bind("user_with_service@example.com")
        .fetch_one(&pool)
        .await
        .unwrap();

    // WHEN
    ServiceRepository::delete_by_id(&pool, service_id)
        .await
        .expect("Impossible de supprimer le service");

    // THEN
    // Verif 1: the service is deleted
    let service_in_db = ServiceRepository::find_by_id(&pool, service_id).await.unwrap();
    assert!(service_in_db.is_none(), "Le service devrait être supprimé");

    // Verif 2: user is disassociated from this service
    let db_service_id_for_user: Option<i64> = sqlx::query_scalar("SELECT service_id FROM user WHERE id = ?")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(db_service_id_for_user.is_none(), "L'id de service pour config du user devrait être mis à NULL");
}
