use backend::service::equivalent_service::EquivalentService;
use sqlx::MySqlPool;
use sqlx::Row;

#[sqlx::test]
async fn devrait_retourner_equivalents_par_defaut_quand_aucun_user(pool: MySqlPool) {
    // GIVEN
    let n = 2;
    let carbon_footprint = 1000.0;

    // WHEN
    let results = EquivalentService::equivalent(&pool, None, n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents par défaut");

    // THEN
    assert_eq!(results.len() as i32, n, "On devrait récupérer le nombre exact d'équivalents demandés");
}

#[sqlx::test]
async fn devrait_retourner_equivalents_par_defaut_quand_user_sans_selection(pool: MySqlPool) {
    // GIVEN
    let row = sqlx::query("SELECT id FROM user WHERE email = 'test_default@example.com'")
        .fetch_one(&pool)
        .await
        .expect("User default not found");
    let user_id: i64 = row.get("id");

    let n = 3;
    let carbon_footprint = 5000.0;

    // WHEN
    let results = EquivalentService::equivalent(&pool, Some(user_id), n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents");

    // THEN
    assert_eq!(results.len() as i32, n, "On devrait récupérer 3 équivalents (fallback défaut)");
}

#[sqlx::test]
async fn devrait_retourner_equivalents_utilisateur_quand_il_a_une_selection(pool: MySqlPool) {
    // GIVEN
    let user_id = 1;
    let n = 5;
    let carbon_footprint = 300000.0;

    // WHEN
    let results = EquivalentService::equivalent(&pool, Some(user_id), n, carbon_footprint)
        .await
        .expect("Erreur lors de la récupération des équivalents de l'utilisateur");

    // THEN
    assert_eq!(results.len(), 2, "Le service devrait retourner uniquement les équivalents sélectionnés par l'utilisateur (ici 2)");

    let names: Vec<String> = results.iter().map(|r| r.name.clone()).collect();
    assert!(names.contains(&"data.equivalent.lille_marseille".to_string()));
    assert!(names.contains(&"data.equivalent.emails".to_string()));
}

#[sqlx::test]
async fn devrait_mettre_a_jour_les_equivalents_utilisateur(pool: MySqlPool) {
    // GIVEN
    let user_id = 1; // User existant avec selection [1, 3]
    let new_selection = vec![2, 4, 5]; // IDs valides dans la migration

    // WHEN
    let update_result = EquivalentService::update_user_equivalents(&pool, user_id, new_selection.clone()).await;

    // THEN
    assert!(update_result.is_ok(), "La mise à jour devrait réussir");

    let selections = EquivalentService::get_all_equivalents_with_selection(&pool, user_id).await.unwrap();

    let selected_count = selections.iter().filter(|s| s.is_selected).count();
    assert_eq!(selected_count, 3, "Il devrait y avoir maintenant 3 équivalents sélectionnés");

    let selected_ids: Vec<i64> = selections.iter().filter(|s| s.is_selected).map(|s| s.id).collect();
    assert!(selected_ids.contains(&2));
    assert!(selected_ids.contains(&4));
    assert!(selected_ids.contains(&5));
    assert!(!selected_ids.contains(&1));
}

#[sqlx::test]
async fn devrait_recuperer_tous_les_equivalents_avec_selection_utilisateur(pool: MySqlPool) {
    // GIVEN
    let user_id = 1;

    // WHEN
    let selections = EquivalentService::get_all_equivalents_with_selection(&pool, user_id)
        .await
        .expect("Erreur lors de la récupération des sélections");

    // THEN
    assert_eq!(selections.len(), 10, "On devrait récupérer la liste complète des équivalents");

    let item_1 = selections.iter().find(|s| s.id == 1).expect("Item 1 présent");
    assert!(item_1.is_selected, "L'item 1 devrait être sélectionné (d'après migration)");

    let item_3 = selections.iter().find(|s| s.id == 3).expect("Item 3 présent");
    assert!(item_3.is_selected, "L'item 3 devrait être sélectionné (d'après migration)");

    let item_2 = selections.iter().find(|s| s.id == 2).expect("Item 2 présent");
    assert!(!item_2.is_selected, "L'item 2 ne devrait pas être sélectionné");
}
