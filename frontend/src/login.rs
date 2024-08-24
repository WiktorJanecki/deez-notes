use crate::{Context, API_PATH};
use anyhow::{bail, Ok, Result};
use gloo_storage::Storage;
use leptos::*;
use reqwest::*;
use serde::{Deserialize, Serialize};

pub const AUTH_TOKEN: &str = "AUTH_TOKEN";

#[derive(Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct LoginResponse {
    #[allow(unused)]
    pub success: bool,
    pub token: String,
}
#[component]
pub fn Login() -> impl IntoView {
    let ctx = expect_context::<Context>();
    let login_signal = ctx.login_signal;
    let error_signal = ctx.error_signal;
    let username = create_rw_signal(String::from(""));
    let password = create_rw_signal(String::from(""));

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error_signal.set(String::from(""));
        spawn_local(async move {
            fetch_login_safe(username.get(), password.get(), login_signal, error_signal).await;
        });
    };
    let on_input = move |ev, signal: RwSignal<String>| {
        signal.set(event_target_value(&ev));
        error_signal.set(String::from(""));
    };

    view! {
        <form on:submit=on_submit>
            <label for="login">"Login: "</label>
            <input on:input=move|ev|{on_input(ev,username)} required  id="login" name="login"/>
            <br/>
            <label for="password">"Password: "</label>
            <input on:input=move|ev|{on_input(ev,password)} name="password" id="password" required type="password"/>
            <br/>
            <button type="submit">"Submit"</button>
            <p>"Don't have an account? "<a href="/register">"Register"</a></p>
        </form>
    }
}

async fn fetch_login_safe(
    username: String,
    password: String,
    login_signal: RwSignal<bool>,
    error_signal: RwSignal<String>,
) {
    let result = fetch_login(username, password, login_signal).await;
    if let Err(err) = result {
        error_signal.set(err.to_string());
    }
}

async fn fetch_login(
    username: String,
    password: String,
    login_signal: RwSignal<bool>,
) -> Result<()> {
    let payload = LoginPayload { username, password };
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{API_PATH}/login"))
        .json(&payload)
        .fetch_credentials_include()
        .send()
        .await?;
    let status = res.status();
    match status {
        StatusCode::OK => {
            let token = res.json::<LoginResponse>().await.unwrap();
            gloo_storage::LocalStorage::set(AUTH_TOKEN, token.token).unwrap();
            login_signal.set(true);
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        }
        _ => {
            let error_message = res.text().await?;
            bail!(error_message);
        }
    };
    Ok(())
}
