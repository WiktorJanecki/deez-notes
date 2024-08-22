use crate::API_PATH;
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
    let (err, set_err) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());
    let login_signal = expect_context::<RwSignal<bool>>();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let payload = LoginPayload {
                username: name.get_untracked(),
                password: pass.get_untracked(),
            };
            let client = reqwest::Client::new();
            let res = client
                .post(format!("{API_PATH}/login"))
                .json(&payload)
                .fetch_credentials_include()
                .send()
                .await
                .expect("API SHOULD BE ALIVE TODO: HANDLE THIS ERR");
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
                    let e = res.text().await.unwrap();
                    set_err.set(e);
                }
            };
        });
    };
    view! {
        <form on:submit=on_submit>
            <label for="login">"Login: "</label>
            <input on:input=move|ev|{
                set_name.set(event_target_value(&ev));
                set_err.set(String::from(""));
            } required  id="login" name="login" value=name/>
            <br/>
            <label for="password">"Password: "</label>
            <input on:input=move|ev|{
                set_pass.set(event_target_value(&ev));
                set_err.set(String::from(""));
            } name="password" id="password" value=pass required type="password"/>
            <br/>
            <button type="submit">"Submit"</button>
            <p class="err">{move||err.get()}</p>
            <p>"Don't have an account? "<a href="/register">"Register"</a></p>
        </form>
    }
}
