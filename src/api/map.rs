use url::Url;
use crate::base::location::LOCATION_STATE;
use jsonwebtoken::crypto::sign;

pub fn generate_url(isDark: bool) -> String {
    let colorScheme = if isDark {
        "dark"
    } else {
        "light"
    };
    let url_base = format!("https://snapshot.apple-mapkit.com/api/v1/snapshot?z=12&scale=2&size=600x200&colorScheme={colorScheme}");

    let mut url =  Url::parse(&url_base).unwrap();

    let apple_team_id = std::env::var("APPLE_DEVELOPER_TEAM_ID").expect("an APPLE_DEVELOPER_TEAM_ID env var");
    let mapkit_key_id = std::env::var("APPLE_MAPKIT_KEY_ID").expect("an APPLE_MAPKIT_KEY_ID env var");
    let mapkit_key_private = std::env::var("APPLE_MAPKIT_KEY_PRIVATE").expect("an APPLE_MAPKIT_KEY_PRIVATE env var");

    if let Some(loc) = LOCATION_STATE.lock().unwrap().as_ref() {
        let center = format!("{}, {}, {}", loc.city, loc.state, loc.country);
        url.query_pairs_mut().append_pair("center", &center);
    }

    url.query_pairs_mut().append_pair("teamId", &apple_team_id);
    url.query_pairs_mut().append_pair("keyId", &mapkit_key_id);

    let snapshot_path = format!("{}?{}", url.path(), url.query().unwrap());

    let key = jsonwebtoken::EncodingKey::from_ec_pem(mapkit_key_private.as_bytes()).expect("a valid JWT key");
    let signature = sign(snapshot_path.as_str().as_bytes(), &key, jsonwebtoken::Algorithm::ES256).expect("a valid ES256 signature");

    url.query_pairs_mut().append_pair("signature", &signature);

    url.to_string()
}
