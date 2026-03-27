use backend::dto::update_account_request_dto::UpdateAccountRequest;
use backend::models::user::User;
use backend::repository::user_repository::UserRepository;
use backend::service::user_service::UserService;
use sqlx::{MySqlPool, Row};

#[sqlx::test]
async fn devrait_confirmer_email_existant(pool: MySqlPool) {
    // GIVEN
    let email = "existing_user_service@test.com";
    UserRepository::insert_user(&pool, email, "pwd", "Jean", "Dupont")
        .await
        .expect("Failed to insert user");

    // WHEN
    let exists = UserService::user_with_email_exists(&pool, email.to_string()).await;

    // THEN
    assert!(exists, "L'email inséré devrait être détecté comme existant");
}

#[sqlx::test]
async fn devrait_confirmer_email_inexistant(pool: MySqlPool) {
    // GIVEN
    let email = "missing_user_service@test.com";

    // WHEN
    let exists = UserService::user_with_email_exists(&pool, email.to_string()).await;

    // THEN
    assert!(!exists, "Un email non présent ne doit pas être considéré existant");
}

#[sqlx::test]
async fn devrait_mettre_a_jour_utilisateur_et_hasher_mot_de_passe(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "update_service_old@test.com", "old_pwd", "Old", "User")
        .await
        .expect("Failed to insert user");

    let current_user = User {
        id: user_id,
        id_service: None,
        email: "update_service_old@test.com".to_string(),
        prenom: "Old".to_string(),
        nom: "User".to_string(),
        total_carbon_footprint: 0.0,
    };

    let payload = UpdateAccountRequest {
        email: Some("update_service_new@test.com".to_string()),
        prenom: Some("Nouveau".to_string()),
        nom: Some("Nom".to_string()),
        password: Some("motdepasse123".to_string()),
    };

    // WHEN
    let updated = UserService::update_user(&pool, current_user, payload)
        .await
        .expect("La mise a jour devrait reussir");

    // THEN
    assert_eq!(updated.email, "update_service_new@test.com");
    assert_eq!(updated.prenom, "Nouveau");
    assert_eq!(updated.nom, "Nom");

    let (_id, stored_hash, first_name, last_name, _service_id) =
        UserRepository::find_with_password_by_email(&pool, "update_service_new@test.com")
            .await
            .expect("Failed to find updated user")
            .expect("Updated user not found");

    assert_eq!(first_name, "Nouveau");
    assert_eq!(last_name, "Nom");
    assert_ne!(stored_hash, "motdepasse123", "Le mot de passe doit etre hashe");
    assert!(
        bcrypt::verify("motdepasse123", &stored_hash).expect("bcrypt verify failed"),
        "Le hash stocke doit correspondre au mot de passe fourni"
    );
}

#[sqlx::test]
async fn devrait_refuser_mise_a_jour_si_email_deja_utilise(pool: MySqlPool) {
    // GIVEN
    UserRepository::insert_user(&pool, "taken_email@test.com", "pwd", "Taken", "User")
        .await
        .expect("Failed to insert existing user");

    let user_id = UserRepository::insert_user(&pool, "editable_user@test.com", "pwd", "Edit", "User")
        .await
        .expect("Failed to insert editable user");

    let current_user = User {
        id: user_id,
        id_service: None,
        email: "editable_user@test.com".to_string(),
        prenom: "Edit".to_string(),
        nom: "User".to_string(),
        total_carbon_footprint: 0.0,
    };

    let payload = UpdateAccountRequest {
        email: Some("taken_email@test.com".to_string()),
        prenom: None,
        nom: None,
        password: None,
    };

    // WHEN
    let result = UserService::update_user(&pool, current_user, payload).await;

    // THEN
    assert_eq!(result.unwrap_err(), "Cet email est déjà utilisé");
}

#[sqlx::test]
async fn devrait_rejoindre_organisation_avec_code_valide(pool: MySqlPool) {
    // GIVEN
    let user_id = 2; // test_default@example.com n'est membre d'aucune orga dans la migration

    // WHEN
    let joined_org = UserService::join_organization(&pool, "TEST_ORG".to_string(), user_id)
        .await
        .expect("Le join devrait reussir")
        .expect("Une organisation devrait etre retournee");

    // THEN
    assert_eq!(joined_org.id, 1);
    assert_eq!(joined_org.code, "TEST_ORG");

    let membership = sqlx::query("SELECT est_admin FROM organisation_user WHERE user_id = ? AND organisation_id = ?")
        .bind(user_id)
        .bind(1)
        .fetch_optional(&pool)
        .await
        .expect("Failed to query organisation_user");

    assert!(membership.is_some(), "Le user devrait etre lie a l'organisation");
    let est_admin: bool = membership.unwrap().try_get("est_admin").expect("Failed to get est_admin");
    assert!(!est_admin, "Le join via ce service doit ajouter un membre non admin");
}

#[sqlx::test]
async fn devrait_refuser_rejoindre_organisation_code_invalide(pool: MySqlPool) {
    // GIVEN
    let user_id = 2;

    // WHEN
    let result = UserService::join_organization(&pool, "UNKNOWN_CODE".to_string(), user_id).await;

    // THEN
    assert_eq!(result.unwrap_err(), "Code d'organisation invalide");
}

#[sqlx::test]
async fn devrait_refuser_rejoindre_organisation_deja_membre(pool: MySqlPool) {
    // GIVEN
    let user_id = 1; // deja membre de TEST_ORG dans la migration

    // WHEN
    let result = UserService::join_organization(&pool, "TEST_ORG".to_string(), user_id).await;

    // THEN
    assert_eq!(result.unwrap_err(), "Déjà membre de cette organisation");
}

#[sqlx::test]
async fn devrait_recuperer_membres_organisation(pool: MySqlPool) {
    // GIVEN
    let orga_id = 1;

    // WHEN
    let members = UserService::get_organization_members(&pool, orga_id)
        .await
        .expect("Failed to get organization members");

    // THEN
    assert!(members.iter().any(|m| m.id == 1), "Le user seed id=1 doit etre present");
    let user_1 = members.into_iter().find(|m| m.id == 1).expect("Le membre id=1 devrait exister");
    assert_eq!(user_1.service_name, Some("Test Service".to_string()));
}

#[sqlx::test]
async fn devrait_supprimer_membre_organisation_et_retirer_service_associe(pool: MySqlPool) {
    // GIVEN
    let user_id = 1;
    let orga_id = 1;

    // WHEN
    UserService::remove_organization_member(&pool, user_id, orga_id)
        .await
        .expect("La suppression du membre devrait reussir");

    // THEN
    let service_id_after: Option<i64> = sqlx::query_scalar("SELECT service_id FROM user WHERE id = ?")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to query user service_id");
    assert!(service_id_after.is_none(), "Le service doit etre retire pour cette organisation");

    let membership_after = sqlx::query("SELECT 1 FROM organisation_user WHERE user_id = ? AND organisation_id = ?")
        .bind(user_id)
        .bind(orga_id)
        .fetch_optional(&pool)
        .await
        .expect("Failed to query membership after delete");
    assert!(membership_after.is_none(), "Le lien organisation_user doit etre supprime");
}

#[sqlx::test]
async fn devrait_supprimer_utilisateur(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "delete_user_service@test.com", "pwd", "To", "Delete")
        .await
        .expect("Failed to insert user");

    // WHEN
    UserService::delete_user(&pool, user_id)
        .await
        .expect("La suppression devrait reussir");

    // THEN
    let found = UserRepository::find_id_by_email(&pool, "delete_user_service@test.com".to_string())
        .await
        .expect("Failed to query user after delete");
    assert!(found.is_none(), "L'utilisateur ne doit plus exister");
}


#[sqlx::test]
async fn devrait_retourner_erreur_mise_a_jour_si_pool_ferme(pool: MySqlPool) {
    // GIVEN
    let user = User {
        id: 1,
        id_service: Some(1),
        email: "test@example.com".to_string(),
        prenom: "Test".to_string(),
        nom: "User".to_string(),
        total_carbon_footprint: 100.0,
    };

    let payload = UpdateAccountRequest {
        email: None,
        prenom: Some("PrenomFerme".to_string()),
        nom: None,
        password: None,
    };

    pool.close().await;

    // WHEN
    let result = UserService::update_user(&pool, user, payload).await;

    // THEN
    let error_message = result.unwrap_err();
    assert!(
        error_message.starts_with("Erreur mise à jour:"),
        "Le message devrait être préfixé par 'Erreur mise à jour:', obtenu: {}",
        error_message
    );
}
