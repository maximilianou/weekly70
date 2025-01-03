use axum::{handler::get, response::Html, routing::post, Router};
use leptos::*;
use lettre::{Message, SmtpTransport, Transport};
use rand::{distributions::Alphanumeric, Rng};
use std::net::SocketAddr;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home_page))
        .route("/login", get(login_page).post(handle_login))
        .route("/recover", post(handle_recovery));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home_page() -> Html<String> {
    let home_html = r#"
    <html>
        <body>
            <h1>Welcome to the Home Page</h1>
            <a href='/login'>Login</a>
        </body>
    </html>"#;
    Html(home_html.to_string())
}

async fn login_page() -> Html<String> {
    let login_html = r#"
    <html>
        <body>
            <h1>Login Page</h1>
            <form action="/login" method="post">
                <label for="username">Username:</label>
                <input type="text" id="username" name="username" required><br>
                <label for="password">Password:</label>
                <input type="password" id="password" name="password" required><br>
                <button type="submit">Login</button>
            </form>
            <form action="/recover" method="post">
                <h2>Forgot Password?</h2>
                <label for="email">Email:</label>
                <input type="email" id="email" name="email" required><br>
                <button type="submit">Recover</button>
            </form>
        </body>
    </html>"#;
    Html(login_html.to_string())
}

async fn handle_login() -> Html<String> {
    // For simplicity, this is just a stub
    Html("<h1>Login successful!</h1>".to_string())
}

async fn handle_recovery(form: axum::extract::Form<RecoveryForm>) -> Html<String> {
    let email = &form.email;
    let recovery_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let email_content = format!(
        "Hello,\n\nYour recovery code is: {}\n\nThank you!",
        recovery_code
    );

    match send_email(email, &email_content).await {
        Ok(_) => Html("<h1>Recovery email sent!</h1>".to_string()),
        Err(_) => Html("<h1>Failed to send recovery email.</h1>".to_string()),
    }
}

async fn send_email(to: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("YourApp <crearbuenosaires@gmail.com>".parse()?)
        .to(to.parse()?)
        .subject("Password Recovery Code")
        .body(body.to_string())?;

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(lettre::transport::smtp::authentication::Credentials::new(
            "crearbuenosaires@gmail.com".to_string(),
            "your-app-password".to_string(),
        ))
        .build();

    mailer.send(&email)?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct RecoveryForm {
    email: String,
}
