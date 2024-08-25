use crate::login::{LoginPayload, LoginResponse, AUTH_TOKEN};
use crate::{Context, API_PATH};
use anyhow::{bail, Ok, Result};
use gloo_storage::Storage;
use leptos::*;
use reqwest::StatusCode;

#[component]
pub fn Register() -> impl IntoView {
    let ctx = expect_context::<Context>();
    let login_signal = ctx.login_signal;
    let err_signal = ctx.error_signal;
    let username = create_rw_signal(String::from(""));
    let password = create_rw_signal(String::from(""));

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        err_signal.set(String::from(""));
        spawn_local(register_safe(
            username.get(),
            password.get(),
            login_signal,
            err_signal,
        ));
    };

    let on_input = move |ev, signal: RwSignal<String>| {
        signal.set(event_target_value(&ev));
        err_signal.set("".to_owned());
    };

    view! {
        <form on:submit=on_submit>
            <label for="login">"Login: "</label>
            <br/>
            <input on:input=move|ev|{on_input(ev,username)} required  id="login" name="login"/>
            <br/>
            <label for="password">"Password: "</label>
            <br/>
            <input on:input=move|ev|{on_input(ev,password)} name="password" id="password" required type="password"/>
            <br/>
            <button type="submit">"Submit"</button>
        </form>
    }
}

async fn register_safe(
    username: String,
    password: String,
    login_signal: RwSignal<bool>,
    error_signal: RwSignal<String>,
) {
    if let Err(e) = register(username, password, login_signal).await {
        error_signal.set(e.to_string());
    }
}

async fn register(username: String, password: String, login_signal: RwSignal<bool>) -> Result<()> {
    let payload = LoginPayload { username, password };
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/register", *API_PATH))
        .json(&payload)
        .fetch_credentials_include()
        .send()
        .await?;
    let status = res.status();
    match status {
        StatusCode::OK => {
            let token = res.json::<LoginResponse>().await.unwrap();
            gloo_storage::LocalStorage::set(AUTH_TOKEN, token.token).unwrap();
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
            login_signal.set(true);
        }
        _ => {
            let e = res.text().await?;
            bail!(e);
        }
    };
    Ok(())
}
