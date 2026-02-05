#[cfg(test)]
mod tests {
    use axum::extract::State;
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use backend::controllers::mo_controller::mo;
    use backend::models::{Account, User};
    use rand::Rng;

    // 1. Branche Outer Match: None (Pas de compte en session)
    #[sqlx::test]
    async fn test_mo_no_session(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let result = mo(State(pool), session).await;

        assert_eq!(result.0.success, false);
        assert!(result.0.mo_infos.is_none());
    }

    // 2. Branche Inner Match: Ok(None) (Compte sans organisation)
    #[sqlx::test]
    async fn test_mo_session_user_no_org(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mut rng = rand::rng();
        let user_id: i64 = rng.random_range(1000..9999);

        sqlx::query("INSERT INTO user (id, email, password, organisation_id, roles) VALUES (?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind(format!("solo_{}@test.com", user_id))
            .bind("hash")
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            format!("solo_{}@test.com", user_id),
            "Jean".to_string(),
            "Solo".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = mo(State(pool), session).await;

        // Vérification : success false car pas d'organisation
        assert_eq!(result.0.success, false);
    }

    // 3. Branche Inner Match: Err(_) (Erreur base de données)
    #[sqlx::test]
    async fn test_mo_db_error(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mut rng = rand::rng();
        let user_id: i64 = rng.random_range(1000..9999);
        let org_id: i64 = rng.random_range(1000..9999);

        let mock_account = Account::User(User::new(
            user_id,
            "error@test.com".to_string(),
            "Bug".to_string(),
            "Fix".to_string(),
            Some(org_id)
        ));
        session.insert("account", mock_account).await.unwrap();

        pool.close().await;

        let result = mo(State(pool), session).await;

        assert_eq!(result.0.success, false);
    }

    // 4. Branche Inner Match: Ok(Some(id)) -> Cas Nominal
    #[sqlx::test]
    async fn test_mo_full_success_with_data(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mut rng = rand::rng();
        let org_id: i64 = rng.random_range(1000..9999);
        let user_id: i64 = rng.random_range(1000..9999);

        sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, ?, ?)")
            .bind(org_id)
            .bind(format!("Ma Super Entreprise {}", org_id))
            .bind(format!("X53JSHNL_{}", org_id))
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO user (id, email, password, organisation_id, roles) VALUES (?, ?, ?, ?, '[]')")
            .bind(user_id)
            .bind(format!("ceo_{}@test.com", user_id))
            .bind("hash")
            .bind(org_id)
            .execute(&pool).await.unwrap();

        sqlx::query(
            "INSERT INTO monitored_website (user_id   , url_full, url_domain, queries_quantity, carbon_footprint, data_transferred, loading_time, country, creation_date)
             VALUES (?, 'https://site.com', 'site.com', 100, 150.5, 500.0, 2.5, 'FR', NOW())"
        )
            .bind(user_id)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            format!("ceo_{}@test.com", user_id),
            "Jean".to_string(),
            "CEO".to_string(),
            Some(org_id)
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = mo(State(pool), session).await;

        assert!(result.0.success);
        assert!(result.0.mo_infos.is_some());
    }

    // 5. Test complémentaire : Succès mais sans consommation (couvre la logique interne de calcul)
    #[sqlx::test]
    async fn test_mo_zero_consumption(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mut rng = rand::rng();
        let org_id: i64 = rng.random_range(10000..19999);
        let user_id: i64 = rng.random_range(10000..19999);

        sqlx::query("INSERT INTO organisation (id, organisation_name, organisation_code) VALUES (?, ?, ?)")
            .bind(org_id)
            .bind(format!("Clean Startup {}", org_id))
            .bind(format!("CLEAN{}", org_id))
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO user (id, email, password, organisation_id, roles) VALUES (?, ?, ?, ?, '[]')")
            .bind(user_id)
            .bind(format!("clean_{}@test.com", user_id))
            .bind("hash")
            .bind(org_id)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            format!("clean_{}@test.com", user_id),
            "Mr".to_string(),
            "Clean".to_string(),
            Some(org_id)
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = mo(State(pool), session).await;

        assert!(result.0.success);
        assert!(result.0.equivalents.is_none());
    }

    // 6. Test Phantom Organization: Couvre l'erreur dans organization_informations
    #[sqlx::test]
    async fn test_mo_phantom_organization(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mut rng = rand::rng();
        let phantom_org_id: i64 = rng.random_range(90000..99999);
        let user_id: i64 = rng.random_range(90000..99999);

        let mut conn = pool.acquire().await.unwrap();

        sqlx::query("SET FOREIGN_KEY_CHECKS=0")
            .execute(&mut *conn).await.unwrap();

        sqlx::query("INSERT INTO user (id, email, password, organisation_id, roles) VALUES (?, ?, ?, ?, '[]')")
            .bind(user_id)
            .bind(format!("ghost_{}@test.com", user_id))
            .bind("hash")
            .bind(phantom_org_id) // ID INEXISTANT
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SET FOREIGN_KEY_CHECKS=1")
            .execute(&mut *conn).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            format!("ghost_{}@test.com", user_id),
            "Casper".to_string(),
            "Ghost".to_string(),
            Some(phantom_org_id)
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = mo(State(pool), session).await;

        assert_eq!(result.0.success, true);
        assert!(result.0.mo_infos.is_none());
        assert!(result.0.letter.is_none());
    }

    // 7. Test Explicit Coverage: Couvre explicitement le bloc Ok(None) (Lignes 239-250)
    #[sqlx::test]
    async fn test_mo_explicit_no_org_coverage(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mock_account = Account::User(User::new(
            888,
            "noorg@test.com".to_string(),
            "No".to_string(),
            "Org".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = mo(State(pool), session).await;

        assert_eq!(result.0.success, false);
        assert!(result.0.mo_infos.is_none());
        assert!(result.0.advices.is_empty());
        assert!(result.0.daily_consumption.is_empty());
    }
}