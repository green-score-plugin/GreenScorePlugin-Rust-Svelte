#[cfg(test)]
mod tests {
    use axum::extract::State;
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use backend::controllers::my_data_controller::my_data;
    use backend::models::{Account, User};

    // Test 1: my_data sans session (utilisateur non connecté)
    #[sqlx::test]
    async fn test_my_data_without_session(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let result = my_data(State(pool), session).await;

        assert!(result.0.success);
        assert!(result.0.my_average_daily_carbon_footprint.is_none());
        assert!(result.0.total_consumption.is_none());
        assert!(result.0.daily_consumption.is_empty());
        assert!(result.0.top_polluting_sites.is_empty());
    }

    // Test 2: my_data avec session mais sans données
    #[sqlx::test]
    async fn test_my_data_with_session_no_data(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool), session).await;

        assert!(result.0.success);
        assert!(result.0.my_average_daily_carbon_footprint.is_none());
        assert!(result.0.daily_consumption.is_empty());
        assert!(result.0.top_polluting_sites.is_empty());
    }

    // Test 3: my_data avec session et données complètes
    #[sqlx::test]
    async fn test_my_data_with_complete_data(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer plusieurs utilisateurs pour le green_score
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(998)
            .bind("test2@test.com")
            .bind("hash")
            .bind("Jane")
            .bind("Smith")
            .execute(&pool).await.unwrap();

        // Insérer des données de monitoring variées
        // Important: créer des données dans les 7 derniers jours pour qu'elles apparaissent dans daily_consumption
        // La requête SQL utilise: creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY)
        sqlx::query("INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
                     (?, 'example.com', 10.0, DATE_SUB(NOW(), INTERVAL 0 DAY)),
                     (?, 'test.com', 15.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 'example.com', 5.0, DATE_SUB(NOW(), INTERVAL 2 DAY)),
                     (?, 'other.com', 8.0, DATE_SUB(NOW(), INTERVAL 3 DAY)),
                     (?, 'site.com', 12.0, DATE_SUB(NOW(), INTERVAL 4 DAY)),
                     (?, 'domain.com', 20.0, DATE_SUB(NOW(), INTERVAL 5 DAY)),
                     (?, NULL, 3.0, DATE_SUB(NOW(), INTERVAL 6 DAY))")
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .bind(user_id)
            .execute(&pool).await.unwrap();

        // Données pour l'autre utilisateur (pour green_score)
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES (?, 100.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(998)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool), session).await;

        // Assertions
        assert!(result.0.success);
        assert!(result.0.my_average_daily_carbon_footprint.is_some());
        assert!(result.0.total_consumption.is_some());
        assert!(!result.0.daily_consumption.is_empty());
        assert!(!result.0.top_polluting_sites.is_empty());
        assert_eq!(result.0.advices.len(), 2);
    }

    // Test 4: my_data avec données pour tester le message average
    #[sqlx::test]
    async fn test_my_data_with_average_message(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer 3 utilisateurs
        for id in [999, 998, 997] {
            sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
                .bind(id)
                .bind(format!("user{}@test.com", id))
                .bind("hash")
                .bind("User")
                .bind(format!("{}", id))
                .execute(&pool).await.unwrap();
        }

        // User 999: faible consommation (5.0)
        // User 998: haute consommation (50.0)
        // User 997: moyenne (20.0)
        // Global avg = (5 + 50 + 20) / 3 = 25
        // User 999: 5 < 25 * 0.8 (20) => "low"
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES
                     (?, 5.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 50.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 20.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(999)
            .bind(998)
            .bind(997)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool), session).await;

        assert!(result.0.success);
        assert_eq!(
            result.0.message_average_footprint.unwrap(),
            "widgets.common.average_daily_footprint.message.low"
        );
    }

    // Test 5: my_data avec équivalents vides (ligne 250 coverage)
    #[sqlx::test]
    async fn test_my_data_with_no_equivalents(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer utilisateurs
        for id in [999, 998] {
            sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
                .bind(id)
                .bind(format!("user{}@test.com", id))
                .bind("hash")
                .bind("User")
                .bind(format!("{}", id))
                .execute(&pool).await.unwrap();
        }

        // Vider la table equivalent
        sqlx::query("DELETE FROM equivalent").execute(&pool).await.ok();

        // Insérer données
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES
                     (?, 100.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 150.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(999)
            .bind(998)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool), session).await;

        assert!(result.0.success);
        // Avec table equivalent vide, equivalents devrait être None (ligne 250 couverte)
        assert!(result.0.equivalents.is_none());
    }

    // Test 6: my_data avec message "average" et "high"
    #[sqlx::test]
    async fn test_my_data_with_average_and_high_messages(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Test "average" : user égal à la moyenne
        for id in [999, 998, 997] {
            sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
                .bind(id)
                .bind(format!("user{}@test.com", id))
                .bind("hash")
                .bind("User")
                .bind(format!("{}", id))
                .execute(&pool).await.unwrap();
        }

        // Tous les users ont la même consommation = 10.0
        // Global avg = 10.0, User 999 = 10.0 => "average"
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES
                     (?, 10.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 10.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 10.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(999)
            .bind(998)
            .bind(997)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool.clone()), session).await;
        assert_eq!(
            result.0.message_average_footprint.unwrap(),
            "widgets.common.average_daily_footprint.message.average"
        );

        // Test "high" dans un second test séparé pour éviter les conflits
        let store2 = Arc::new(MemoryStore::default());
        let session2 = Session::new(Some(Id::default()), store2, None);

        // Créer 3 nouveaux utilisateurs
        for id in [996, 995, 994] {
            sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
                .bind(id)
                .bind(format!("user{}@test.com", id))
                .bind("hash")
                .bind("User")
                .bind(format!("{}", id))
                .execute(&pool).await.unwrap();
        }

        // User 996: haute consommation (100.0)
        // User 995: faible (10.0)
        // User 994: faible (10.0)
        // Global avg = (100 + 10 + 10) / 3 = 40
        // User 996: 100 > 40 * 1.2 (48) => "high"
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES
                     (?, 100.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 10.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
                     (?, 10.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(996)
            .bind(995)
            .bind(994)
            .execute(&pool).await.unwrap();

        let mock_account2 = Account::User(User::new(
            996,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session2.insert("account", mock_account2).await.unwrap();

        let result2 = my_data(State(pool), session2).await;
        assert_eq!(
            result2.0.message_average_footprint.unwrap(),
            "widgets.common.average_daily_footprint.message.high"
        );
    }

    // Test 7: tester les erreurs SQL dans get_my_average_daily_carbon_footprint
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_my_average(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_my_average_daily_carbon_footprint;
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_my_average_daily_carbon_footprint(&pool, session).await;

        // Doit retourner None en cas d'erreur SQL
        assert!(result.is_none());
    }

    // Test 8: tester les erreurs SQL dans get_average_daily_carbon_footprint
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_global_average(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_average_daily_carbon_footprint;

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_average_daily_carbon_footprint(&pool).await;

        // Doit retourner None en cas d'erreur SQL
        assert!(result.is_none());
    }

    // Test 9: tester les erreurs SQL dans get_total_consumption
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_total_consumption(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_total_consumption;
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_total_consumption(&pool, session).await;

        // Doit retourner None en cas d'erreur SQL
        assert!(result.is_none());
    }

    // Test 10: tester la branche else de message_average_footprint (une seule None)
    #[sqlx::test]
    async fn test_my_data_with_only_global_average(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        // Créer plusieurs utilisateurs
        for id in [999, 998] {
            sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
                .bind(id)
                .bind(format!("user{}@test.com", id))
                .bind("hash")
                .bind("User")
                .bind(format!("{}", id))
                .execute(&pool).await.unwrap();
        }

        // Insérer des données seulement pour user 998 (pas pour 999)
        sqlx::query("INSERT INTO monitored_website (user_id, carbon_footprint, creation_date) VALUES (?, 50.0, DATE_SUB(NOW(), INTERVAL 1 DAY))")
            .bind(998)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            user_id,
            "test@test.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            None
        ));
        session.insert("account", mock_account).await.unwrap();

        let result = my_data(State(pool), session).await;

        assert!(result.0.success);
        // User 999 n'a pas de données, donc my_average est None
        assert!(result.0.my_average_daily_carbon_footprint.is_none());
        // Mais il y a une moyenne globale
        assert!(result.0.average_daily_carbon_footprint.is_some());
        // Le message doit être None car my_average est None
        assert!(result.0.message_average_footprint.is_none());
    }

    // Test 11: tester les erreurs SQL dans get_daily_consumption
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_daily_consumption(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_daily_consumption;
        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_daily_consumption(&pool, user_id).await;

        // Doit retourner une erreur
        assert!(result.is_err());
    }

    // Test 12: tester les erreurs SQL dans get_weekly_consumption
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_weekly_consumption(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_weekly_consumption;
        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_weekly_consumption(&pool, user_id).await;

        // Doit retourner une erreur
        assert!(result.is_err());
    }

    // Test 13: tester les erreurs SQL dans get_monthly_consumption
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_monthly_consumption(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_monthly_consumption;
        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_monthly_consumption(&pool, user_id).await;

        // Doit retourner une erreur
        assert!(result.is_err());
    }

    // Test 14: tester les erreurs SQL dans get_top5_polluting_sites
    #[sqlx::test]
    async fn test_my_data_with_sql_error_on_top_polluting_sites(pool: MySqlPool) {
        use backend::controllers::my_data_controller::get_top5_polluting_sites;
        let user_id = 999;

        // Créer un utilisateur
        sqlx::query("INSERT INTO user (id, email, password, first_name, last_name, organisation_id, roles) VALUES (?, ?, ?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .bind("John")
            .bind("Doe")
            .execute(&pool).await.unwrap();

        // Supprimer la table pour causer une erreur SQL
        sqlx::query("DROP TABLE monitored_website").execute(&pool).await.unwrap();

        let result = get_top5_polluting_sites(&pool, user_id).await;

        // Doit retourner une erreur
        assert!(result.is_err());
    }
}

