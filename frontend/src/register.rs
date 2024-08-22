use crate::login::{LoginPayload, LoginResponse, AUTH_TOKEN};
use crate::API_PATH;
use gloo_storage::Storage;
use leptos::*;
use reqwest::StatusCode;

async fn register(
    username: ReadSignal<String>,
    password: ReadSignal<String>,
    login_signal: RwSignal<bool>,
    err_signal: RwSignal<String>,
) {
    let payload = LoginPayload {
        username: username.get(),
        password: password.get(),
    };
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{API_PATH}/register"))
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
            err_signal.set(e);
        }
    };
}

#[component]
pub fn Register() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());
    let err_signal = create_rw_signal(String::from(""));
    let login_signal = expect_context::<RwSignal<bool>>();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(register(name, pass, login_signal, err_signal));
    };
    view! {
        <form on:submit=on_submit>
            <label for="login">"Login: "</label>
            <input on:input=move|ev|{
                set_name.set(event_target_value(&ev));
                err_signal.set(String::from(""));
            } required  id="login" name="login" value=name/>
            <br/>
            <label for="password">"Password: "</label>
            <input on:input=move|ev|{
                set_pass.set(event_target_value(&ev));
                err_signal.set(String::from(""));
            } name="password" id="password" value=pass required type="password"/>
            <br/>
            <button type="submit">"Submit"</button>
            <p class="err">{move||err_signal.get()}</p>
        </form>
    }
}
