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
