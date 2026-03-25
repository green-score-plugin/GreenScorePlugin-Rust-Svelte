use sqlx::MySqlPool;
use backend::repository::organisation_repository::OrganisationRepository;

#[sqlx::test]
async fn devrait_inserer_une_organisation_et_recuperer_son_id(pool: MySqlPool) {
    // GIVEN
    let name = "Org Test Insert";
    let code = "TESTINS";
    let siret = Some("12345678901234".to_string());

    // WHEN
    let id = OrganisationRepository::insert_organisation(&pool, name, code, siret.clone())
        .await
        .expect("Failed to insert organisation");

    // THEN
    let org = OrganisationRepository::find_by_id(&pool, id)
        .await
        .expect("Failed to find organisation")
        .unwrap();

    assert_eq!(org.nom, name);
    assert_eq!(org.code, code);
    assert_eq!(org.siret, siret);
}

#[sqlx::test]
async fn devrait_trouver_id_par_siret(pool: MySqlPool) {
    // GIVEN
    let name = "Org Siret";
    let code = "SIRET01";
    let siret = "98765432109876";

    let id = OrganisationRepository::insert_organisation(&pool, name, code, Some(siret.to_string()))
        .await
        .expect("Failed to insert organisation");

    // WHEN
    let found_id = OrganisationRepository::find_id_by_siret(&pool, siret)
        .await
        .expect("Failed to find by siret");

    // THEN
    assert_eq!(found_id, Some(id));
}

#[sqlx::test]
async fn devrait_trouver_id_par_nom(pool: MySqlPool) {
    // GIVEN
    let name = "Org Name Search";
    let code = "NAME01";

    let id = OrganisationRepository::insert_organisation(&pool, name, code, None)
        .await
        .expect("Failed to insert organisation");

    // WHEN
    let found_id = OrganisationRepository::find_id_by_name(&pool, name)
        .await
        .expect("Failed to find by name");

    // THEN
    assert_eq!(found_id, Some(id));
}


#[sqlx::test]
async fn devrait_mettre_a_jour_une_organisation(pool: MySqlPool) {
    // GIVEN
    let name = "Org Old Name";
    let code = "UPD001";
    let id = OrganisationRepository::insert_organisation(&pool, name, code, None)
        .await
        .expect("Failed to insert organisation");

    let new_name = "Org New Name";
    let new_siret = Some("11223344556677".to_string());

    // WHEN
    OrganisationRepository::update_organisation(&pool, id, new_name, new_siret.clone())
        .await
        .expect("Failed to update organisation");

    // THEN
    let org = OrganisationRepository::find_by_id(&pool, id)
        .await
        .expect("Failed to retrieve organisation")
        .expect("Organisation missing");

    assert_eq!(org.nom, new_name);
    assert_eq!(org.siret, new_siret);
    assert_eq!(org.code, code); // Code unchanged
}

#[sqlx::test]
async fn devrait_trouver_par_code(pool: MySqlPool) {
    // GIVEN
    let name = "Org Code Search";
    let code = "CODEXYZ";
    let id = OrganisationRepository::insert_organisation(&pool, name, code, None)
        .await
        .expect("Failed to insert");

    // WHEN
    let org = OrganisationRepository::find_organization_by_code(&pool, code.to_string())
        .await
        .expect("Failed to find by code")
        .expect("Org missing");

    // THEN
    assert_eq!(org.id, id);
    assert_eq!(org.code, code);
}

#[sqlx::test]
async fn devrait_recuperer_nom_organisation(pool: MySqlPool) {
    // GIVEN
    let name = "Org Name Only";
    let code = "NAMEONLY";
    let id = OrganisationRepository::insert_organisation(&pool, name, code, None)
        .await
        .expect("Failed to insert");

    // WHEN
    let found_name = OrganisationRepository::organization_name(&pool, id)
        .await
        .expect("Failed to get name");

    // THEN
    assert_eq!(found_name, Some(name.to_string()));
}
