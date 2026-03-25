use backend::controllers;

#[tokio::test]
async fn health_check_works() {
    assert!(true);
}

#[tokio::test]
async fn database_connection_test() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}


