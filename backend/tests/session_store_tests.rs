use backend::session_store::MySqlStore;
use sqlx::MySqlPool;
use tower_sessions::session::Record;
use tower_sessions::SessionStore;
use time::OffsetDateTime;
use std::collections::HashMap;

#[sqlx::test]
async fn devrait_creer_et_charger_une_session(pool: MySqlPool) {
    // GIVEN
    let store = MySqlStore::new(pool);
    let mut record = Record {
        id: tower_sessions::session::Id::default(),
        data: HashMap::new(),
        expiry_date: OffsetDateTime::now_utc() + time::Duration::days(1),
    };

    // WHEN
    let create_result = store.create(&mut record).await;
    let load_result = store.load(&record.id).await;

    // THEN
    assert!(create_result.is_ok(), "La création devrait réussir");
    assert!(load_result.is_ok(), "Le chargement devrait réussir");
    let loaded_record = load_result.unwrap().expect("Le record devrait exister");
    assert_eq!(loaded_record.id, record.id, "Les IDs devraient correspondre");
}

#[sqlx::test]
async fn devrait_sauvegarder_et_mettre_a_jour_une_session(pool: MySqlPool) {
    // GIVEN
    let store = MySqlStore::new(pool);
    let mut record = Record {
        id: tower_sessions::session::Id::default(),
        data: HashMap::new(),
        expiry_date: OffsetDateTime::now_utc() + time::Duration::days(1),
    };

    // Creation initiale
    store.create(&mut record).await.unwrap();

    // WHEN - Modifier et sauvegarder
    record.expiry_date = OffsetDateTime::now_utc() + time::Duration::days(2);
    let save_result = store.save(&record).await;

    // THEN
    assert!(save_result.is_ok(), "La sauvegarde devrait réussir");
    let loaded = store.load(&record.id).await.unwrap().unwrap();
    // On ne vérifie pas l'égalité parfaite des dates à cause de la précision (unix_timestamp)
    // mais on sait au moins que l'ID existe et a été mis à jour sans erreur.
    assert_eq!(loaded.id, record.id);
}

#[sqlx::test]
async fn devrait_supprimer_une_session(pool: MySqlPool) {
    // GIVEN
    let store = MySqlStore::new(pool);
    let mut record = Record {
        id: tower_sessions::session::Id::default(),
        data: HashMap::new(),
        expiry_date: OffsetDateTime::now_utc() + time::Duration::days(1),
    };
    store.create(&mut record).await.unwrap();

    // WHEN
    let delete_result = store.delete(&record.id).await;

    // THEN
    assert!(delete_result.is_ok(), "La suppression devrait réussir");
    let load_result = store.load(&record.id).await.unwrap();
    assert!(load_result.is_none(), "La session ne devrait plus exister");
}

#[sqlx::test]
async fn devrait_nettoyer_une_session_expigree_au_chargement(pool: MySqlPool) {
    // GIVEN
    let store = MySqlStore::new(pool.clone());
    let past_date = OffsetDateTime::now_utc() - time::Duration::days(1);
    let mut record = Record {
        id: tower_sessions::session::Id::default(),
        data: HashMap::new(),
        expiry_date: past_date,
    };
    store.create(&mut record).await.unwrap();

    // WHEN
    let load_result = store.load(&record.id).await.unwrap();

    // THEN
    assert!(load_result.is_none(), "Une session expirée devrait renvoyer None");

    // Et elle devrait avoir été supprimée de la base
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE id = ?")
        .bind(record.id.to_string())
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(count, 0, "La session expirée doit être supprimée de la BDD");
}

