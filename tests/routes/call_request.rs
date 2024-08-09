use reqwest::StatusCode;
use scraper::{selectable::Selectable, ElementRef, Html, Selector};

use crate::helpers::{assert_is_redirect_to, TestApp};

#[tokio::test]
async fn home_should_have_link_to_call_request() {
    let app = TestApp::spawn().await;

    let response = app.get_home_page().await;

    assert!(response.status().is_success());

    let link_selector = Selector::parse("a[id='call-request-link']").unwrap();
    let page_doc = Html::parse_document(&response.text().await.unwrap());

    let call_request_link: Vec<ElementRef> = page_doc.select(&link_selector).collect();

    assert!(
        !call_request_link.is_empty(),
        "There should be at least one call request link."
    );

    for link in call_request_link {
        let href = link.attr("href");
        assert!(href.is_some_and(|href| href == "/call_request"))
    }
}

#[tokio::test]
async fn call_request_page_should_have_form() {
    let app = TestApp::spawn().await;

    let response = app.get_call_request_page().await;

    assert!(response.status().is_success());

    let form_selector = Selector::parse("form#call-request-form").unwrap();
    let phone_input_selector = Selector::parse("input#phone").unwrap();
    let name_input_selector = Selector::parse("input#name").unwrap();
    let page_doc = Html::parse_document(&response.text().await.unwrap());

    let call_request_forms: Vec<ElementRef> = page_doc.select(&form_selector).collect();
    assert!(
        !call_request_forms.is_empty(),
        "There should be at least one call request form!"
    );
    for form in call_request_forms {
        assert!(
            form.select(&phone_input_selector).count() == 1,
            "There should be one phone input"
        );
        assert!(
            form.select(&name_input_selector).count() == 1,
            "There should be one name input"
        );
    }
}

#[tokio::test]
async fn malformed_call_requests_are_rejected() {
    let app = TestApp::spawn().await;

    let test_cases = vec![
        (
            serde_json::json!({
                "phone_number": "320 406 7090", // sorry if it's a correcy number.
            }),
            "missing contact_name",
        ),
        (
            serde_json::json!({
                "contact_name": "Gregory Sech",
            }),
            "missing phone_number",
        ),
        (serde_json::json!({}), "empty body"),
    ];
    for (body, description) in test_cases {
        let response = app.post_call_request(&body).await;
        assert!(
            response.status() == StatusCode::BAD_REQUEST,
            "The API did not return BAD_REQUEST when the payload was {}",
            description
        );
    }
}

#[tokio::test]
async fn bad_call_requests_redirect_and_display_error() {
    let app = TestApp::spawn().await;
    let test_cases = vec![
        (
            serde_json::json!({
                "phone_number": "320 406 7090", // sorry if it's a correcy number.
                "contact_name": "a"
            }),
            "bad contact_name",
        ),
        (
            serde_json::json!({
                "contact_name": "Gregory Sech",
                "phone_number": "3"
            }),
            "bad phone_number",
        ),
    ];
    for (body, description) in test_cases {
        let err_response = app.post_call_request(&body).await;
        assert_is_redirect_to(&err_response, "/call_request");
        let response = app.get_call_request_page().await;
        let response_text = response.text().await.unwrap_or_else(move |err| {
            panic!(
                "Could not get response text for test case {}. Error: {}",
                description, err
            )
        });
        assert!(response_text.contains("h2"),);
    }
}

#[tokio::test]
async fn submitting_call_request_form_adds_it_to_db() {
    let app = TestApp::spawn().await;
    let body = serde_json::json!({
        "phone_number": "321 456 7891",
        "contact_name": "Rino Pape",
    });
    let response = app.post_call_request(&body).await;
    assert_is_redirect_to(&response, "/");

    let saved = sqlx::query!("SELECT phone_number, user_name FROM call_requests")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.phone_number, "321 456 7891");
    assert_eq!(saved.user_name, "Rino Pape");
}
