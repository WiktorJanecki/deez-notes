use ev::Event;
use leptos::*;
use leptos_router::{use_params_map, ParamsMap};
use serde::Serialize;

use crate::note::{load_note_data, Note};
use crate::API_PATH;

#[component]
pub fn EditComponent() -> impl IntoView {
    let id = create_rw_signal(0_i32);
    let title = create_rw_signal(String::from(""));
    let content = create_rw_signal(String::from(""));
    let params = use_params_map();
    let on_back = move |_| {
        let navigate = leptos_router::use_navigate();
        navigate("/", Default::default());
    };
    let on_save = move |_| {
        spawn_local(fetch_edit(id.get(), title.get(), content.get()));
    };
    view! {
        <article>
            <Await future=move||fetch_get(params,id,title,content) let:_>
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
async fn fetch_edit(id: i32, new_title: String, new_content: String) {
    let payload = EditPayload {
        title: new_title,
        content: new_content,
    };
    let client = reqwest::Client::new();
    let _response = client
        .put(format!("{API_PATH}/notes/{id}"))
        .json(&payload)
        .fetch_credentials_include()
        .send()
        .await
        .expect("API ERROR");
    let navigate = leptos_router::use_navigate();
    navigate("/", Default::default());
}
async fn fetch_get(
    params: Memo<ParamsMap>,
    id: RwSignal<i32>,
    title: RwSignal<String>,
    content: RwSignal<String>,
) -> Note {
    let note = load_note_data(params, id).await;
    title.set(note.title.clone());
    content.set(note.content.clone());
    note
}
