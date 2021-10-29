use std::str::FromStr;

use reqwest::Url;

use jwks_client_rs::source::WebSource;
use jwks_client_rs::{JsonWebKey, JwksClient, JwksClientError};

#[tokio::main]
async fn main() {
    // This value must be set as one of your tenant key id (in the json: "keys"[0]."kid")
    // export KID={YOUR-KID}
    let kid: String = std::env::var("KID").unwrap();
    // This should be something like
    // export AUTH0_BASE_URL=https://{YOUR-TENANT}.eu.auth0.com
    let url_string: String = std::env::var("AUTH0_BASE_URL").unwrap();

    let url: Url = Url::from_str(url_string.as_str()).unwrap();
    let url: Url = url.join(".well-known/jwks.json").unwrap();
    assert_eq!(
        url.as_str(),
        format!("{}/.well-known/jwks.json", url_string)
    );

    let source: WebSource = WebSource::new(url);
    let client: JwksClient = JwksClient::new(source);

    // The kid "unknown" cannot be a JWKS valid KID. This must not be found here
    let result: Result<JsonWebKey, JwksClientError> = client.get("unknown".to_string()).await;
    println!(
        "Get with kid \"unknown\": {}",
        result.unwrap_err().to_string()
    );

    // The provided kid (assuming is the same you got from your tenant) is valid and could be found.
    let result: Result<JsonWebKey, JwksClientError> = client.get(kid.clone()).await;
    println!("Get with kid \"{}\": {:?}", kid, result.unwrap());

    let result: Result<Claims, JwksClientError> =
        client.decode::<Claims>("token", "audience").await;
}
