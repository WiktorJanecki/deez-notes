use crate::{Context, API_PATH};
use anyhow::bail;
use leptos::*;
use leptos_router::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Note {
    pub id: i32,
    pub creator_id: i32,
    pub title: String,
    pub content: String,
    pub time_created: i64,
    pub time_edited: i64,
}
#[derive(Params, PartialEq)]
pub struct NoteParams {
    id: Option<i32>,
}

#[component]
pub fn NoteTitle(id: i32, title: String) -> impl IntoView {
    let Context { error_signal, .. } = expect_context::<Context>();
    let on_submit = move |_| spawn_local(delete_note_safe(id, error_signal));
    view! {
        <div>
            <a href=format!("/note/{id}")><h4>{title}</h4></a>
            <button on:click={on_submit}>X</button>
        </div>
    }
}

#[component]
pub fn NoteComponent() -> impl IntoView {
    let Context { error_signal, .. } = expect_context::<Context>();
    let id = create_rw_signal(0_i32);
    let params = use_params_map();

    let on_back = move |_| {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    };
    let on_edit = move |_| {
        let navigate = leptos_router::use_navigate();
        let id = id.get();
        navigate(&format!("/edit/{id}"), Default::default());
    };

    view! {
        <article>
            <Await future=move||load_note_data_safe(params,id,error_signal) let:data >
                {
                    match data{
                        Some(note) => view!{
                            <h2>{note.title.to_string()}</h2>
                            <p>{note.content.to_string()}</p>
                            <button on:click=on_back>back</button>
                            <button on:click=on_edit>edit</button>
                        }.into_view(),
                        None => view!{}.into_view()
                    }
                }
            </Await>
        </article>
    }
}

async fn delete_note_safe(id: i32, error_signal: RwSignal<String>) {
    if let Err(e) = delete_note(id).await {
        error_signal.set(e.to_string());
    }
}

async fn delete_note(id: i32) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .delete(format!("{API_PATH}/notes/{id}"))
        .fetch_credentials_include()
        .send()
        .await?;
    if !matches!(res.status(), StatusCode::OK) {
        let e = res.text().await?;
        bail!(e);
    }
    let navigate = leptos_router::use_navigate();
    // TODO: this rerenders home view twice, to fix that make home view data a resource and refetch it
    navigate("/login", Default::default());
    navigate("/", Default::default());
    anyhow::Ok(())
}

pub async fn load_note_data_safe(
    params: Memo<ParamsMap>,
    id: RwSignal<i32>,
    error_signal: RwSignal<String>,
) -> Option<Note> {
    match load_note_data(params, id).await {
        Ok(note) => Some(note),
        Err(e) => {
            error_signal.set(e.to_string());
            None
        }
    }
}

pub async fn load_note_data(params: Memo<ParamsMap>, id: RwSignal<i32>) -> anyhow::Result<Note> {
    let pid = params.with(|params| params.get("id").expect("invalid params").clone());
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{API_PATH}/notes/{pid}"))
        .fetch_credentials_include()
        .send()
        .await?;
    if !matches!(res.status(), StatusCode::OK) {
        let e = res.text().await?;
        bail!(e);
    }
    let note = res.json::<Note>().await?;
    id.set(note.id);
    Ok(note)
}
