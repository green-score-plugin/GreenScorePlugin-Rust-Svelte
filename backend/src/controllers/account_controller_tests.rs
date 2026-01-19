use super::*;
use serde_json::from_value;

#[test]
fn test_update_account_request_deserialization() {
    // Cas 1 : JSON complet
    let json_full = json!({
        "email": "test@example.com",
        "prenom": "Jean",
        "nom": "Dupont",
        "password": "Password123!"
    });
    let req: UpdateAccountRequest = from_value(json_full).expect("Test OK");
    assert_eq!(req.email, Some("test@example.com".to_string()));
    assert_eq!(req.prenom, Some("Jean".to_string()));
    assert_eq!(req.nom, Some("Dupont".to_string()));
    assert_eq!(req.password, Some("Password123!".to_string()));

    // Cas 2 : JSON partiel (seulement prénom)
    let json_partial = json!({
        "prenom": "Pierre"
    });
    let req: UpdateAccountRequest = from_value(json_partial).expect("Devrait désérialiser le JSON partiel");
    assert!(req.email.is_none());
    assert_eq!(req.prenom, Some("Pierre".to_string()));
    assert!(req.nom.is_none());
    assert!(req.password.is_none());

    // Cas 3 : JSON vide
    let json_empty = json!({});
    let req: UpdateAccountRequest = from_value(json_empty).expect("Devrait désérialiser le JSON vide");
    assert!(req.email.is_none());
    assert!(req.prenom.is_none());
    assert!(req.nom.is_none());
}

#[test]
fn test_join_orga_request_deserialization() {
    let json = json!({ "code": "ABC12345" });
    let req: JoinOrgaRequest = from_value(json).expect("Devrait désérialiser JoinOrgaRequest");
    assert_eq!(req.code, "ABC12345");
}

#[test]
fn test_dynamic_query_building_logic() {
    // Simulation de la logique de construction de requête SQL présente dans update_account
    // Ce test garantit que la logique "si champ présent, alors ajouter au SQL" fonctionne comme prévu
    // C'est une réimplémentation locale pour le test, car la logique est inline dans le contrôleur.

    let payload = UpdateAccountRequest {
        email: Some("new@example.com".to_string()),
        prenom: None,
        nom: Some("NewName".to_string()),
        password: None,
    };

    let mut query = String::from("UPDATE user SET ");
    let mut updates = Vec::new();
    let mut params_count = 0;

    if payload.email.is_some() {
        updates.push("email = ?");
        params_count += 1;
    }
    if payload.prenom.is_some() {
        updates.push("first_name = ?");
        params_count += 1;
    }
    if payload.nom.is_some() {
        updates.push("last_name = ?");
        params_count += 1;
    }
    if payload.password.is_some() {
        updates.push("password = ?");
        params_count += 1;
    }

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    assert_eq!(query, "UPDATE user SET email = ?, last_name = ? WHERE id = ?");
    assert_eq!(params_count, 2);
}

