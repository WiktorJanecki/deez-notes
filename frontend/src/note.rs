use crate::API_PATH;
use ev::SubmitEvent;
use leptos::*;
use leptos_dom::logging::console_log;
use leptos_router::*;
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
    let on_submit = move |_| spawn_local(delete_note(id));
    view! {
        <div>
            <a href=format!("/note/{id}")><h4>{title}</h4></a>
            <button on:click={on_submit}>X</button>
        </div>
    }
}

#[component]
pub fn NoteComponent() -> impl IntoView {
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
            <Await future=move||load_note_data(params,id) let:data >
                <h2>{data.title.to_string()}</h2>
                <p>{data.content.to_string()}</p>
                <button on:click=on_back>back</button>
                <button on:click=on_edit>edit</button>
            </Await>
        </article>
    }
}

async fn delete_note(id: i32) {
    let client = reqwest::Client::new();
    let _res = client
        .delete(format!("{API_PATH}/notes/{id}"))
        .fetch_credentials_include()
        .send()
        .await
        .expect("api error");
    let navigate = leptos_router::use_navigate();
    // TODO: this rerenders home view twice, to fix that make home view data a resource and refetch it
    navigate("/login", Default::default());
    navigate("/", Default::default());
}

pub async fn load_note_data(params: Memo<ParamsMap>, id: RwSignal<i32>) -> Note {
    let pid = params.with(|params| params.get("id").expect("invalid params").clone());
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{API_PATH}/notes/{pid}"))
        .fetch_credentials_include()
        .send()
        .await
        .expect("api error")
        .json::<Note>()
        .await
        .unwrap();
    id.set(res.id);
    res
}
