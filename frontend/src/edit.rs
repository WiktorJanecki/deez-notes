use anyhow::bail;
use leptos::*;
use leptos_router::{use_params_map, ParamsMap};
use reqwest::StatusCode;
use serde::Serialize;

use crate::note::load_note_data;
use crate::{Context, API_PATH};

#[component]
pub fn EditComponent() -> impl IntoView {
    let Context { error_signal, .. } = expect_context::<Context>();
    let id = create_rw_signal(0_i32);
    let title = create_rw_signal(String::from(""));
    let content = create_rw_signal(String::from(""));
    let params = use_params_map();

    let on_back = move |_| {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    };

    let on_save = move |_| {
        spawn_local(fetch_edit_safe(
            id.get(),
            title.get(),
            content.get(),
            error_signal,
        ));
    };

    view! {
        <article>
            <Await future=move||fetch_get_safe(params,id,title,content,error_signal) let:_>
                <input on:input=move|e|{on_input(e,title)} value={title} />
                <br/>
                <textarea on:input=move|e|{on_input(e,content)} prop:value={content}/>
                <br/>
                <button on:click=on_back>back</button>
                <button on:click=on_save>save</button>
            </Await>
        </article>
    }
}

fn on_input(e: ev::Event, s: RwSignal<String>) {
    s.set(event_target_value(&e));
}
#[derive(Serialize)]
struct EditPayload {
    title: String,
    content: String,
}
async fn fetch_edit_safe(
    id: i32,
    new_title: String,
    new_content: String,
    error_signal: RwSignal<String>,
) {
    if let Err(e) = fetch_edit(id, new_title, new_content).await {
        error_signal.set(e.to_string());
    }
}
async fn fetch_edit(id: i32, new_title: String, new_content: String) -> anyhow::Result<()> {
    let payload = EditPayload {
        title: new_title,
        content: new_content,
    };
    let client = reqwest::Client::new();
    let res = client
        .put(format!("{API_PATH}/notes/{id}"))
        .json(&payload)
        .fetch_credentials_include()
        .send()
        .await?;
    if !matches!(res.status(), StatusCode::OK) {
        let e = res.text().await?;
        bail!(e);
    }
    let navigate = leptos_router::use_navigate();
    navigate("/", Default::default());
    Ok(())
}
async fn fetch_get_safe(
    params: Memo<ParamsMap>,
    id: RwSignal<i32>,
    title: RwSignal<String>,
    content: RwSignal<String>,
    error_signal: RwSignal<String>,
) {
    let res = load_note_data(params, id).await;
    match res {
        Ok(note) => {
            title.set(note.title.clone());
            content.set(note.content.clone());
        }
        Err(e) => {
            error_signal.set(e.to_string());
        }
    };
}
