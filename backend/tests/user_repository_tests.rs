use sqlx::MySqlPool;
use sqlx::Row;
use backend::repository::user_repository::UserRepository;
use backend::repository::organisation_repository::OrganisationRepository;
use backend::models::user::User;

#[sqlx::test]
async fn devrait_inserer_et_trouver_user(pool: MySqlPool) {
    // GIVEN
    let email = "test_insert@example.com";
    let password = "hashed_password";
    let first_name = "Jean";
    let last_name = "Dupont";

    // WHEN
    let user_id = UserRepository::insert_user(&pool, email, password, first_name, last_name)
        .await
        .expect("Failed to insert user");

    // THEN
    let found_id = UserRepository::find_id_by_email(&pool, email.to_string())
        .await
        .expect("Failed to find user")
        .expect("User not found");

    assert_eq!(user_id, found_id);
}

#[sqlx::test]
async fn devrait_trouver_avec_mot_de_passe(pool: MySqlPool) {
    // GIVEN
    let email = "test_pwd@example.com";
    let password = "secret_password";
    UserRepository::insert_user(&pool, email, password, "Alice", "Wonder")
        .await
        .expect("Failed to insert user");

    // WHEN
    let result = UserRepository::find_with_password_by_email(&pool, email)
        .await
        .expect("Failed to find user with password");

    // THEN
    assert!(result.is_some());
    let (_id, pwd, first_name, last_name, srv_id) = result.unwrap();
    assert_eq!(pwd, password);
    assert_eq!(first_name, "Alice");
    assert_eq!(last_name, "Wonder");
    assert_eq!(srv_id, None);
}

#[sqlx::test]
async fn devrait_rejoindre_organisation(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "join_org@test.com", "pwd", "Bob", "Builder")
        .await
        .expect("Failed to insert user");

    let org_id = OrganisationRepository::insert_organisation(&pool, "Org Join", "JOIN_CODE", None)
        .await
        .expect("Failed to insert org");

    // WHEN
    UserRepository::join_organisation(&pool, user_id, org_id, true)
        .await
        .expect("Failed to join organisation");

    // THEN
    // Verify in organisation_user table directly as User struct doesn't hold this anymore
    let row = sqlx::query("SELECT est_admin FROM organisation_user WHERE user_id = ? AND organisation_id = ?")
        .bind(user_id)
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        .expect("Failed to query organisation_user");

    assert!(row.is_some());
    let is_admin: bool = row.unwrap().try_get("est_admin").expect("Failed to get est_admin");
    assert!(is_admin);
}

#[sqlx::test]
async fn devrait_mettre_a_jour_empreinte_carbone(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "carbon@test.com", "pwd", "Green", "Man")
        .await
        .expect("Failed to insert user");

    // WHEN
    UserRepository::update_total_carbon_footprint_by_id(&pool, user_id, 10.5)
        .await
        .expect("Failed to update carbon");

    // Update again to test accumulation if logic supports it (query says: COALESCE(..., 0) + ?)
    UserRepository::update_total_carbon_footprint_by_id(&pool, user_id, 5.5)
        .await
        .expect("Failed to update carbon again");

    // THEN
    let carbon = UserRepository::find_total_carbon_footprint_by_id(&pool, user_id)
        .await
        .expect("Failed to get carbon")
        .unwrap();

    assert_eq!(carbon, 16.0);
}

#[sqlx::test]
async fn devrait_mettre_a_jour_user(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "update@test.com", "old_pwd", "Old", "Name")
        .await
        .expect("Failed to insert user");

    let user_struct = User {
        id: user_id,
        email: "updated@test.com".to_string(),
        prenom: "New".to_string(),
        nom: "NameUpdated".to_string(),
        id_service: None,
        total_carbon_footprint: 0.0,
    };

    // WHEN
    // Update with new password
    UserRepository::update_user(&pool, user_struct.clone(), Some("new_pwd".to_string()))
        .await
        .expect("Failed to update user");

    // THEN
    let (id, pwd, fname, lname, _) = UserRepository::find_with_password_by_email(&pool, "updated@test.com")
        .await
        .expect("Failed to find user")
        .unwrap();

    assert_eq!(id, user_id);
    assert_eq!(pwd, "new_pwd");
    assert_eq!(fname, "New");
    assert_eq!(lname, "NameUpdated");
}

#[sqlx::test]
async fn devrait_supprimer_user(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "delete@test.com", "pwd", "Del", "Me")
        .await
        .expect("Failed to insert user");

    // WHEN
    UserRepository::delete_user(&pool, user_id)
        .await
        .expect("Failed to delete user");

    // THEN
    let found = UserRepository::find_id_by_email(&pool, "delete@test.com".to_string())
        .await
        .expect("Failed to find user");

    assert!(found.is_none());
}

#[sqlx::test]
async fn devrait_gerer_membres_organisation(pool: MySqlPool) {
    // GIVEN
    let org_id = OrganisationRepository::insert_organisation(&pool, "Members Org", "MEMBERS", None)
        .await
        .expect("Failed to insert org");

    let u1 = UserRepository::insert_user(&pool, "u1@org.com", "p", "U", "One")
        .await.expect("Insert u1");
    let u2 = UserRepository::insert_user(&pool, "u2@org.com", "p", "U", "Two")
        .await.expect("Insert u2");

    // Add to org
    UserRepository::update_user_organization(&pool, u1, org_id).await.expect("Add u1");
    UserRepository::update_user_organization(&pool, u2, org_id).await.expect("Add u2");

    // WHEN
    let members = UserRepository::get_organization_members(&pool, org_id)
        .await
        .expect("Failed to get members");

    // THEN
    assert_eq!(members.len(), 2);
    assert!(members.iter().any(|u| u.id == u1));
    assert!(members.iter().any(|u| u.id == u2));

    // WHEN remove u1
    UserRepository::remove_organization_member(&pool, u1, org_id)
        .await
        .expect("Failed to remove member");

    // THEN
    let members_after = UserRepository::get_organization_members(&pool, org_id)
        .await
        .expect("Failed to get members after removal");

    assert_eq!(members_after.len(), 1);
    assert_eq!(members_after[0].id, u2);
}

#[sqlx::test]
async fn devrait_compter_equivalents_utilisateur(pool: MySqlPool) {
    // GIVEN
    let user_id = UserRepository::insert_user(&pool, "eq@test.com", "p", "Eq", "User")
        .await
        .expect("Failed to insert user");

    let _ = sqlx::query("INSERT IGNORE INTO equivalent (id, name, equivalent, icon_thumbnail) VALUES (1, 'e1', 1.0, 'i1')")
        .execute(&pool)
        .await;

    sqlx::query("INSERT INTO user_equivalent (user_id, equivalent_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(1)
        .execute(&pool)
        .await
        .expect("Failed to insert user_equivalent");

    // WHEN
    let count = UserRepository::count_user_equivalent(&pool, user_id)
        .await
        .expect("Failed to count");

    // THEN
    assert_eq!(count, 1);
}
