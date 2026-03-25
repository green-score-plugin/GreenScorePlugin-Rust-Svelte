use backend::service::equivalent_service::EquivalentService;
use sqlx::MySqlPool;

// --- Helper pour insérer des données de test ---
async fn setup_equivalent_data(pool: &MySqlPool) -> (i64, Vec<i64>) {
    // Insérer un utilisateur factice
    let user_res = sqlx::query("INSERT INTO user (email, password, roles, total_carbon_footprint, est_admin) VALUES (?, ?, '[]', 0, 0)")
        .bind("equiv_service_test@example.com")
        .bind("password")
        .execute(pool)
        .await
        .expect("Failed to insert user");
    let user_id = user_res.last_insert_id() as i64;

    // Insérer quelques équivalents
    let mut eq_ids = vec![];
    for i in 1..=3 {
        let eq_res = sqlx::query("INSERT INTO equivalent (name, equivalent, icon_thumbnail) VALUES (?, ?, ?)")
            .bind(format!("Equiv {}", i))
            .bind(10.0 * i as f64)
            .bind("icon.png")
            .execute(pool)
            .await
            .expect("Failed to insert equivalent");
        eq_ids.push(eq_res.last_insert_id() as i64);
    }

    (user_id, eq_ids)
}

#[sqlx::test]
async fn devrait_retourner_equivalents_par_defaut_quand_aucun_user(pool: MySqlPool) {
    // GIVEN
    setup_equivalent_data(&pool).await;
    let n = 2;
    let carbon_footprint = 1000.0; // 1kg

    // WHEN
    let results = EquivalentService::equivalent(&pool, None, n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents par défaut");

    // THEN
    assert_eq!(results.len() as i32, n, "On devrait récupérer le nombre n d'équivalents demandés");
}

#[sqlx::test]
async fn devrait_retourner_equivalents_par_defaut_quand_user_sans_selection(pool: MySqlPool) {
    // GIVEN
    let (user_id, _) = setup_equivalent_data(&pool).await;
    let n = 2;
    let carbon_footprint = 5000.0;

    // WHEN
    // L'utilisateur existe mais n'a pas d'équivalents associés
    let results = EquivalentService::equivalent(&pool, Some(user_id), n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents pour utilisateur sans sélection");

    // THEN
    assert!(results.len() > 0, "On devrait récupérer un fallback sur les équivalents par défaut");
}

#[sqlx::test]
async fn devrait_retourner_equivalents_utilisateur_quand_il_a_une_selection(pool: MySqlPool) {
    // GIVEN
    let (user_id, eq_ids) = setup_equivalent_data(&pool).await;

    // Assigner le premier équivalent à l'utilisateur
    sqlx::query("INSERT INTO user_equivalent (user_id, equivalent_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(eq_ids[0])
        .execute(&pool)
        .await
        .expect("Failed to link equivalent to user");

    let n = 5;
    let carbon_footprint = 10000.0;

    // WHEN
    let results = EquivalentService::equivalent(&pool, Some(user_id), n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents de l'utilisateur");

    // THEN
    assert_eq!(results.len(), 1, "Le service devrait retourner le seul équivalent de l'utilisateur");
    assert_eq!(results[0].name, "Equiv 1", "Le nom devrait correspondre à l'équivalent sélectionné");
}

#[sqlx::test]
async fn devrait_mettre_a_jour_les_equivalents_utilisateur(pool: MySqlPool) {
    // GIVEN
    let (user_id, eq_ids) = setup_equivalent_data(&pool).await;
    let new_selection = vec![eq_ids[1], eq_ids[2]];

    // WHEN
    let update_result = EquivalentService::update_user_equivalents(&pool, user_id, new_selection.clone()).await;

    // THEN
    assert!(update_result.is_ok(), "La mise à jour devrait réussir");

    let selections = EquivalentService::get_all_equivalents_with_selection(&pool, user_id).await.unwrap();

    let selected_count = selections.iter().filter(|s| s.is_selected).count();
    assert_eq!(selected_count, 2, "Il devrait y avoir 2 équivalents sélectionnés");
}

#[sqlx::test]
async fn devrait_recuperer_tous_les_equivalents_avec_selection_utilisateur(pool: MySqlPool) {
    // GIVEN
    let (user_id, eq_ids) = setup_equivalent_data(&pool).await;
    EquivalentService::update_user_equivalents(&pool, user_id, vec![eq_ids[0]]).await.unwrap();

    // WHEN
    let selections = EquivalentService::get_all_equivalents_with_selection(&pool, user_id)
        .await
        .expect("Erreur lors de la récupération des sélections");

    // THEN
    assert!(selections.len() >= 3, "Il devrait y avoir au moins nos 3 équivalents de test");

    let selected_item = selections.iter().find(|s| s.id == eq_ids[0]).expect("Item 1 devrait être là");
    assert!(selected_item.is_selected, "L'item 1 devrait être marqué comme sélectionné");

    let unselected_item = selections.iter().find(|s| s.id == eq_ids[1]).expect("Item 2 devrait être là");
    assert!(!unselected_item.is_selected, "L'item 2 ne devrait pas être marqué comme sélectionné");
}
