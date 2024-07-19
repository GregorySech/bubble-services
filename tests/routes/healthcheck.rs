use crate::helpers::TestApp;

#[tokio::test]
async fn health_check_works() {
    let app = TestApp::spawn().await;

    let response = app
        .http_client
        .get(&format!("{}/healthcheck", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
