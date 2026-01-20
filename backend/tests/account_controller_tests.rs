use backend::controllers::account_controller::{UpdateAccountRequest, JoinOrgaRequest};
use serde_json::{from_value, json};

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

    let payload = UpdateAccountRequest {
        email: Some("new@example.com".to_string()),
        nom: Some("NewName".to_string()),
        ..Default::default()
    };

    let (query, params_count) = payload.build_update_query();

    assert_eq!(query, "UPDATE user SET email = ?, last_name = ? WHERE id = ?");
    assert_eq!(params_count, 2);
}

