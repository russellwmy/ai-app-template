use slack_morphism::{signature_verifier::SlackEventSignatureVerifier, SlackSigningSecret};

pub fn verify_api_gateway_req(req: &lambda_http::Request) {
    let slack_signing_secret =
        std::env::var("SLACK_SIGNING_SECRET").expect("No SLACK_SIGNING_SECRET set in env!");

    let headers = req.headers();

    let body_as_string =
        String::from_utf8(req.body().to_vec()).expect("Unable to convert APIG Event to string");

    let timestamp = headers[SlackEventSignatureVerifier::SLACK_SIGNED_TIMESTAMP]
        .to_str()
        .expect("header not a string");

    let signature = headers[SlackEventSignatureVerifier::SLACK_SIGNED_HASH_HEADER]
        .to_str()
        .expect("header not a string");
    let signing_secret = SlackSigningSecret::new(slack_signing_secret);

    SlackEventSignatureVerifier::new(&signing_secret)
        .verify(signature, &body_as_string, timestamp)
        .expect("Verificaction failed, cannnot trust API Gateway Request is from Slack");
}
