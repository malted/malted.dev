use once_cell::sync::Lazy;
use reqwest::header::HeaderMap;
use reqwest::Client;
use rocket::form::Form;
use rocket::http::{ContentType, Status};
use rocket::response::content::RawHtml;
use serde::Deserialize;
use serde_json::{json, Value};
use std::env::var;

static LOOPS_CLIENT: Lazy<Client> = Lazy::new(|| {
    let builder = reqwest::ClientBuilder::new();

    let loops_api_key = var("LOOPS_API_KEY").expect("a LOOPS_API_KEY env var");

    let mut headers = HeaderMap::new();
    headers.append(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {loops_api_key}").parse().unwrap(),
    );
    headers.append(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );

    builder
        .default_headers(headers)
        .build()
        .expect("a built loops client")
});

static SLACK_CLIENT: Lazy<Client> = Lazy::new(|| {
    let builder = reqwest::ClientBuilder::new();

    let slack_bot_token = var("SLACK_BOT_TOKEN").expect("a SLACK_BOT_TOKEN env var");

    let mut headers = HeaderMap::new();
    headers.append(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {slack_bot_token}").parse().unwrap(),
    );
    headers.append(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    builder
        .default_headers(headers)
        .build()
        .expect("a built slack client")
});

#[get("/")]
pub fn form() -> RawHtml<&'static str> {
    RawHtml(
        r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Subscribe</title>
        <meta name="viewport" content="width=device-width, initial-scale=1">
    </head>
    <body>
        <h1>Message in a bottle</h1>
        <p>Malted's little newsletter.</p>
        <form action="/bottle/" method="post">
            <label>
                Email
                <input type="email" name="email" required>
            </label>
            <br>
            <label>
                First name
                <input type="text" name="first_name">
            </label>
            <br>
            <label>
                Last name
                <input type="text" name="last_name">
            </label>
            <br>
            <button type="submit">Submit</button>
        </form>
        <br />
        <img width=256 src='https://hc-cdn.hel1.your-objectstorage.com/s/v3/a7d5ae7afab39c75be02be977624a075c54361b8_esnupi-seaside.jpg' alt='esnupi, seaside' />
    </body>
</html>
"#,
    )
}

async fn sign_up(
    email: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
    source: &str,
) -> Result<(), reqwest::Error> {
    let body = json!({
        "email": email,
        "firstName": first_name,
        "lastName": last_name,
        "source": source,
    });

    println!("{body}");

    LOOPS_CLIENT
        .put("https://app.loops.so/api/v1/contacts/update")
        .json(&body)
        .send()
        .await?;

    Ok(())
}

#[derive(FromForm, Debug)]
pub struct UserInput<'r> {
    email: &'r str,
    first_name: Option<&'r str>,
    last_name: Option<&'r str>,
}

#[post("/", data = "<user_input>")]
pub async fn subscribe(user_input: Form<UserInput<'_>>) -> Result<RawHtml<&'static str>, Status> {
    println!("{:?}", user_input);
    match sign_up(
        user_input.email,
        user_input.first_name,
        user_input.last_name,
        "malted.dev",
    )
    .await
    {
        Ok(_) => Ok(RawHtml(
            r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Subscribed</title>
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                </head>
                <body>
                    <h1>Thank you for subscribing!</h1>
                    <p>You've been added to my little newsletter.</p>
                    <a href="/bottle">Back</a>
                    <br />
                    <a href="/">Go to malted.dev</a>
                </body>
            </html>
            "#,
        )),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Debug, Deserialize, FromForm)]
pub struct SlackSlashCommandInput<'a> {
    user_id: &'a str,
}

#[post("/slack", data = "<input>")]
pub async fn slack_slash_command_handler(
    input: Form<SlackSlashCommandInput<'_>>,
) -> Result<&'static str, Status> {
    let resp = SLACK_CLIENT
        .get(format!(
            "https://slack.com/api/users.info?user={}",
            input.user_id
        ))
        .send()
        .await
        .map_err(|e| {
            error!("couldn’t reach Slack: {e}");
            Status::ServiceUnavailable
        })?;

    let slack: SlackUserResponse = resp.json().await.map_err(|e| {
        error!("invalid JSON from Slack: {e}");
        Status::ServiceUnavailable
    })?;

    if !slack.ok {
        error!("Slack replied with ok=false");
        return Err(Status::BadRequest);
    }
    let profile = slack.user.profile;
    let email = profile.email;

    if email.is_empty() {
        Err(Status::BadRequest)?
    }

    sign_up(
        &email,
        profile.first_name.as_deref(),
        profile.last_name.as_deref(),
        "slack",
    )
    .await
    .map_err(|e| {
        error!("Loops sign‑up failed: {e}");
        Status::ServiceUnavailable
    })?;

    Ok("You'll hear from me soon!")
}

#[derive(Debug, Deserialize)]
struct SlackUserResponse {
    ok: bool,
    user: SlackUser,
}

#[derive(Debug, Deserialize)]
struct SlackUser {
    profile: SlackProfile,
}

#[derive(Debug, Deserialize)]
struct SlackProfile {
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
}
