#[cfg(test)]
mod tests {
    use backend::service::organisation_service::OrganisationService;
    use backend::dto::update_organisation_request_dto::UpdateOrganisationRequest;
    use backend::models::organisation::Organisation;
    use sqlx::MySqlPool;

    #[sqlx::test]
    async fn devrait_trouver_id_par_siret(pool: MySqlPool) {
        // GIVEN
        let siret = "12345678901234".to_string();

        // WHEN
        let org_id = OrganisationService::find_id_by_siret(&pool, siret).await.unwrap();

        // THEN
        assert_eq!(org_id, Some(1), "Devrait retourner l'ID 1 pour le siret existant");
    }

    #[sqlx::test]
    async fn devrait_retourner_informations_organisation(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;
        let user_id = 1;

        // WHEN
        let infos = OrganisationService::organization_informations(&pool, org_id, user_id).await.unwrap();

        // THEN
        assert_eq!(infos.name, "Test Organisation");
        assert!(infos.total_consumption >= 0.0);
    }

    #[tokio::test]
    async fn devrait_generer_code_organisation() {
        // WHEN
        let code = OrganisationService::generate_organisation_code();

        // THEN
        assert_eq!(code.len(), 8, "Le code devrait faire 8 caractères de long");
    }

    #[sqlx::test]
    async fn devrait_mettre_a_jour_informations_organisation(pool: MySqlPool) {
        // GIVEN
        let current_org = Organisation {
            id: 1,
            nom: "Test Organisation".to_string(),
            code: "TEST_ORG".to_string(),
            siret: Some("12345678901234".to_string()),
            est_admin: true,
        };
        let payload = UpdateOrganisationRequest {
            id: Some(1),
            name: "Nouveau Nom Organisation".to_string(),
            siret: Some("98765432109876".to_string()),
        };

        // WHEN
        let updated_org = OrganisationService::update_organisation_details(&pool, &current_org, payload).await.unwrap();

        // THEN
        assert_eq!(updated_org.nom, "Nouveau Nom Organisation");
        assert_eq!(updated_org.siret, Some("98765432109876".to_string()));
    }

    #[sqlx::test]
    async fn devrait_inscrire_nouvelle_organisation(pool: MySqlPool) {
        // GIVEN
        let user_id = 2; // user "Default User" is not in any org right now
        let nom = "Ma Nouvelle Super Orga";
        let siret = Some("nouv-siret".to_string());

        // WHEN
        let (org_id, code) = OrganisationService::inscription_orga(&pool, nom, siret, user_id).await.unwrap();

        // THEN
        assert!(org_id > 1, "Devrait créer une organisation avec un nouvel ID");
        assert_eq!(code.len(), 8, "Le code généré devrait faire 8 caractères");
    }

    #[sqlx::test]
    async fn devrait_supprimer_organisation(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;

        // WHEN
        let res = OrganisationService::delete_organization(&pool, org_id).await;

        // THEN
        assert!(res.is_ok(), "La suppression devrait réussir");
    }

    #[sqlx::test]
    async fn devrait_retourner_daily_organization_consumption(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;

        // WHEN
        let res = OrganisationService::get_daily_organization_consumption(&pool, org_id).await;

        // THEN
        assert!(res.is_ok(), "Devrait récupérer les données journalières");
    }

    #[sqlx::test]
    async fn devrait_retourner_weekly_organization_consumption(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;

        // WHEN
        let res = OrganisationService::get_weekly_organization_consumption(&pool, org_id).await;

        // THEN
        assert!(res.is_ok(), "Devrait récupérer les données hebdomadaires");
    }

    #[sqlx::test]
    async fn devrait_retourner_monthly_organization_consumption(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;

        // WHEN
        let res = OrganisationService::get_monthly_organization_consumption(&pool, org_id).await;

        // THEN
        assert!(res.is_ok(), "Devrait récupérer les données mensuelles");
    }

    #[sqlx::test]
    async fn devrait_retourner_top5_polluting_sites_organization(pool: MySqlPool) {
        // GIVEN
        let org_id = 1;

        // On insère un site pour l'utilisateur 1 (qui appartient à l'organisation 1)
        // afin d'avoir une vraie valeur de consommation à mapper et arrondir.
        sqlx::query(
            "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES (?, ?, ?)")
            .bind(1)
            .bind("example.com")
            .bind(1.23456)
            .execute(&pool)
            .await
            .unwrap();

        // WHEN
        let res = OrganisationService::get_top5_polluting_sites_by_organization(&pool, org_id).await;

        // THEN
        assert!(res.is_ok(), "Devrait récupérer le top 5 des sites polluants");
        let sites = res.unwrap();
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].total_footprint, 1.23, "La valeur de carbon_footprint devrait être arrondie à deux décimales");
    }
}
