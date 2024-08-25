use anyhow::{bail, Result};
use gloo_storage::Storage;
use leptos::*;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    login::AUTH_TOKEN,
    note::{Note, NoteTitle},
    Context, API_PATH,
};

#[component]
pub fn Home() -> impl IntoView {
    let ctx = expect_context::<Context>();
    let login_signal = ctx.login_signal;
    let error_signal = ctx.error_signal;

    // run at start
    create_effect(move |_| {
        let token = gloo_storage::LocalStorage::get::<String>(AUTH_TOKEN).ok();
        // TODO: check on server if token is still valid
        if token.is_some() {
            login_signal.set(true);
        }
    });

    let on_click = move |_| {
        spawn_local(fetch_new_note_safe(error_signal));
    };

    move || match login_signal.get() {
        true => view! {<>
            <Await future= move||fetch_notes_safe(error_signal) let:data>
            <h3>"Notes: "</h3>
            <article>
                {
                    data.iter()
                    .map(|note| view!{<NoteTitle id=note.id title=note.title.clone()/>})
                    .collect::<Vec<_>>()
                }
                <button on:click={on_click}>Add new note</button>
            </article>
            </Await>
        </>},
        false => view! {<><h3>"Must be logged in"</h3></>},
    }
}

#[derive(Serialize)]
struct CreatePayload {
    title: String,
    content: String,
}

async fn fetch_new_note_safe(error_signal: RwSignal<String>) {
    if let Err(err) = fetch_new_note().await {
        error_signal.set(err.to_string());
    }
}
async fn fetch_new_note() -> Result<()> {
    let payload = CreatePayload {
        title: "New Note".to_owned(),
        content: "Sample content".to_owned(),
    };
    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/notes", *API_PATH))
        .json(&payload)
        .fetch_credentials_include()
        .send()
        .await?;
    if !matches!(res.status(), StatusCode::OK) {
        let e = res.text().await?;
        bail!(e);
    }
    let navigate = leptos_router::use_navigate();
    let id = res.json::<Note>().await?.id;
    navigate(&format!("/edit/{id}"), Default::default());
    Ok(())
}

async fn fetch_notes_safe(error_signal: RwSignal<String>) -> Vec<Note> {
    match fetch_notes().await {
        Result::Ok(v) => v,
        Result::Err(e) => {
            error_signal.set(e.to_string());
            vec![]
        }
    }
}

async fn fetch_notes() -> Result<Vec<Note>> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/notes", *API_PATH))
        .fetch_credentials_include()
        .send()
        .await?;
    if !matches!(res.status(), StatusCode::OK) {
        let e = res.text().await?;
        bail!(e);
    }
    let vector = res.json::<Vec<Note>>().await?;
    Ok(vector)
}
