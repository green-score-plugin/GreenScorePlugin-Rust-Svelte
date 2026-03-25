use sqlx::MySqlPool;
use backend::repository::equivalent_repository::EquivalentRepository;
use backend::repository::user_repository::UserRepository;

#[sqlx::test]
async fn devrait_retourner_3_equivalent_get_n_equivalent(pool: MySqlPool) {
    // GIVEN
    let n: i32 = 3;

    // WHEN
    let equivalents = EquivalentRepository::get_n_equivalent(&pool, n, 1.0)
        .await
        .expect("Failed to get equivalents");

    // THEN
    assert_eq!(equivalents.len(), 3,
               "La taille de la variable 'equivalents' devrait être égale à 3");
}

#[sqlx::test]
async fn devrait_retourner_emails_user_equivalent(pool: MySqlPool) {
    // GIVEN
    // Create a user first
    let user_id = UserRepository::insert_user(&pool, "eq_test1@example.com", "pwd", "Eq", "Test1")
        .await
        .expect("Failed to insert user");

    // Insert user equivalent (assuming equivalent.emails ID is 3)
    let email_eq_id = 3;
    sqlx::query("INSERT INTO user_equivalent (user_id, equivalent_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(email_eq_id)
        .execute(&pool)
        .await
        .expect("Failed to insert user_equivalent");

    let n: i32 = 1;
    let expected: &str = "data.equivalent.emails";

    // WHEN
    let equivalents = EquivalentRepository::get_n_user_equivalent(&pool, user_id, n, 1.0)
        .await
        .expect("Failed to get user equivalents");

    // THEN
    assert_eq!(equivalents.len(), 1,
        "La longueur de la variable 'equivalents' devrait être 1");
    assert_eq!(equivalents[0].name, expected,
        "L'équivalent retourné devrait être 'data.equivalent.emails'")
}

#[sqlx::test]
async fn devrait_retourner_2_equivalents_selectionne_get_all_equivalents_with_selection(pool: MySqlPool) {
    // GIVEN
    // Create a user first
    let user_id = UserRepository::insert_user(&pool, "eq_test2@example.com", "pwd", "Eq", "Test2")
        .await
        .expect("Failed to insert user");

    let expected_selected = vec![
        "data.equivalent.lille_marseille",
        "data.equivalent.emails"
    ];

    // Insert user equivalents manually (IDs 1 and 3 based on migration)
    for eq_id in [1, 3] {
        sqlx::query("INSERT INTO user_equivalent (user_id, equivalent_id) VALUES (?, ?)")
            .bind(user_id)
            .bind(eq_id)
            .execute(&pool)
            .await
            .expect("Failed to insert user_equivalent");
    }

    // WHEN
    let all_equivalents = EquivalentRepository::get_all_equivalents_with_selection(&pool, user_id)
        .await
        .expect("Failed to get user equivalents with selection");
    let selected_equivalents = all_equivalents.iter().filter(|(_, _, _, is_selected)| *is_selected).collect::<Vec<_>>();

    // THEN
    assert_eq!(selected_equivalents.len(), 2,
        "La longueur de la variable 'selected_equivalents' devrait être 2");
    for (_, name, _, _) in selected_equivalents {
        assert!(expected_selected.contains(&name.as_str()),
            "L'équivalent sélectionné '{}' devrait être dans la liste des équivalents attendus", name);
    }
}

#[sqlx::test]
async fn devrait_mettre_a_jour_les_equivalents_update_user_equivalents(pool: MySqlPool) {
    // GIVEN
    // Create a user first
    let user_id = UserRepository::insert_user(&pool, "eq_test3@example.com", "pwd", "Eq", "Test3")
        .await
        .expect("Failed to insert user");

    let new_equivalents_id = vec![2, 4, 5];

    // WHEN
    EquivalentRepository::update_user_equivalents(&pool, user_id, new_equivalents_id.clone())
        .await
        .expect("Failed to update user equivalents");
    let new_equivalents = EquivalentRepository::get_all_equivalents_with_selection(&pool, user_id)
        .await
        .expect("Failed to get new user equivalents with selection");
    let selected_equivalents: Vec<i64> = new_equivalents
        .iter()
        .filter(|(_, _, _, is_selected)| *is_selected)
        .map(|(id, _, _, _)| *id)
        .collect();

    // THEN
    assert_eq!(selected_equivalents.len(), new_equivalents_id.len(),
        "Le nombre d'équivalents sélectionnés devrait être égal à {}", new_equivalents_id.len());

    for id in &new_equivalents_id {
        assert!(selected_equivalents.contains(id),
            "L'équivalent avec l'id '{}' devrait être dans la liste des équivalents sélectionnés", id);
    }
}