use sqlx::MySqlPool;
use backend::service::auth_service::AuthService;
use backend::error::AppError;

#[sqlx::test]
async fn devrait_inscrire_un_nouvel_utilisateur(pool: MySqlPool) {
    // GIVEN
    let email = "new_user@example.com";
    let password = "mon_super_password";
    let first_name = "Jean";
    let last_name = "Dupont";

    // WHEN
    let result = AuthService::inscription(&pool, email, password, first_name, last_name).await;

    // THEN
    assert!(result.is_ok(), "L'inscription devrait réussir");
    let user_id = result.unwrap();
    assert!(user_id > 0, "L'ID de l'utilisateur devrait être > 0");
}

#[sqlx::test]
async fn ne_devrait_pas_inscrire_un_utilisateur_existant(pool: MySqlPool) {
    // GIVEN - L'utilisateur "test@example.com" existe déjà via la migration
    let email = "test@example.com";
    let password = "peu_importe";
    let first_name = "Test";
    let last_name = "User";

    // WHEN
    let result = AuthService::inscription(&pool, email, password, first_name, last_name).await;

    // THEN
    assert!(result.is_err(), "L'inscription devrait échouer pour un email existant");
    assert_eq!(result.unwrap_err(), "errors.auth.email_exists");
}

#[sqlx::test]
async fn devrait_connecter_un_utilisateur_existant(pool: MySqlPool) {
    // GIVEN
    let email = "login_success@example.com";
    let password = "password_secure";

    // Inscription préalable
    AuthService::inscription(&pool, email, password, "Alice", "Wonder")
        .await
        .expect("L'inscription a échoué");

    // WHEN
    let result = AuthService::login(&pool, email, password).await;

    // THEN
    assert!(result.is_ok(), "La connexion devrait réussir avec le bon email et le bon mot de passe");
    let user_full = result.unwrap();
    assert_eq!(user_full.user.email, email);
    assert_eq!(user_full.user.prenom, "Alice");
    assert_eq!(user_full.user.nom, "Wonder");
}

#[sqlx::test]
async fn ne_devrait_pas_connecter_un_utilisateur_avec_mauvais_mot_de_passe(pool: MySqlPool) {
    // GIVEN
    let email = "wrong_pwd@example.com";
    let password = "good_password";
    let wrong_password = "bad_password";

    // Inscription préalable
    AuthService::inscription(&pool, email, password, "Bob", "Build")
        .await
        .expect("L'inscription a échoué");

    // WHEN
    let result = AuthService::login(&pool, email, wrong_password).await;

    // THEN
    assert!(result.is_err(), "La connexion devrait échouer avec le mauvais mot de passe");
    match result.unwrap_err() {
        AppError::AuthError(msg) => assert_eq!(msg, "errors.auth.invalid_credentials"),
        _ => panic!("Le type d'erreur n'est pas AppError::AuthError"),
    }
}

#[sqlx::test]
async fn ne_devrait_pas_connecter_un_utilisateur_inexistant(pool: MySqlPool) {
    // WHEN
    let result = AuthService::login(&pool, "nexiste_pas@example.com", "random_pwd").await;

    // THEN
    assert!(result.is_err(), "La connexion devrait échouer pour un utilisateur inexistant");
    match result.unwrap_err() {
        AppError::AuthError(msg) => assert_eq!(msg, "errors.auth.invalid_credentials"),
        _ => panic!("Le type d'erreur n'est pas AppError::AuthError"),
    }
}
