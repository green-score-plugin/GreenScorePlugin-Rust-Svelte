use backend::models::Organisation;

#[test]
fn test_organisation_creation() {
    let org = Organisation::new(
        1,
        "Acme Corp".to_string(),
        Some("12345678901234".to_string()),
        "ABC123".to_string(),
        42
    );

    assert_eq!(org.id, 1);
    assert_eq!(org.nom, "Acme Corp");
    assert_eq!(org.siret, Some("12345678901234".to_string()));
    assert_eq!(org.code, "ABC123");
    assert_eq!(org.admin_id, 42);
}

#[test]
fn test_organisation_sans_siret() {
    let org = Organisation::new(
        2,
        "StartUp".to_string(),
        None,
        "XYZ789".to_string(),
        10
    );

    assert_eq!(org.id, 2);
    assert_eq!(org.siret, None);
}

#[test]
fn test_organisation_serialization() {
    let org = Organisation::new(
        3,
        "Test Org".to_string(),
        Some("98765432109876".to_string()),
        "TEST01".to_string(),
        5
    );

    let json = serde_json::to_string(&org).unwrap();
    assert!(json.contains("Test Org"));
    assert!(json.contains("98765432109876"));
}

#[test]
fn test_organisation_deserialization() {
    let json = r#"{"id":4,"nom":"Demo Corp","siret":"11111111111111","code":"DEMO","admin_id":7}"#;
    let org: Organisation = serde_json::from_str(json).unwrap();

    assert_eq!(org.id, 4);
    assert_eq!(org.nom, "Demo Corp");
    assert_eq!(org.admin_id, 7);
}
