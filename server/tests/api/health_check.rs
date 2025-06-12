// tests/api/health_check.rs

// integration test which checks the /health_check endpoint

// dependencies
use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_returns_200_ok_and_empty_body() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .client
        .get(format!("{}/api/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}