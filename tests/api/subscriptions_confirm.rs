use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = spawn_app().await;
    let body = "name=test%20user&email=test_user%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(&email_request);

    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn confirmations_with_an_invalid_token_format_are_rejected_with_a_400() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("a".repeat(24), "was too short"),
        ("a".repeat(26), "was too long"),
        ("*".repeat(25), "contained forbidden characters"),
    ];

    for (invalid_token, error_message) in test_cases {
        let response = reqwest::get(&format!(
            "{}/subscriptions/confirm?subscription_token={}",
            app.address, invalid_token
        ))
        .await
        .unwrap();

        assert_eq!(
            response.status().as_u16(),
            400,
            "The API did not fail with 400 Bad Request when the payload {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn confirmations_with_well_formatted_but_non_existent_token_are_rejected_with_a_404() {
    let app = spawn_app().await;

    let response = reqwest::get(&format!(
        "{}/subscriptions/confirm?subscription_token={}",
        app.address,
        "a".repeat(25)
    ))
    .await
    .unwrap();

    assert_eq!(response.status().as_u16(), 404);
}
