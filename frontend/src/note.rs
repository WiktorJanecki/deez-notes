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
struct NoteParams {
    id: Option<i32>,
}

#[component]
pub fn note_title(id: i32, title: String) -> impl IntoView {
    let on_submit = move |_| {
        spawn_local(async move {
            let client = reqwest::Client::new();
            let _res = client
                .delete(format!("{API_PATH}/notes/{id}"))
                .fetch_credentials_include()
                .send()
                .await
                .expect("api error");
            let navigate = leptos_router::use_navigate();
            navigate("/login", Default::default());
            let navigate = leptos_router::use_navigate();
            navigate("/", Default::default());
        });
    };
    view! {
        <div>
            <a href=format!("/note/{id}")><h4>{title}</h4></a>
            <button on:click={on_submit}>X</button>
        </div>
    }
}
#[component]
pub fn NoteComponent() -> impl IntoView {
    let title = create_rw_signal(String::from(""));
    let content = create_rw_signal(String::from(""));
    let id = create_rw_signal(0_i32);

    let params = use_params_map();
    spawn_local(async move {
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
        title.set(res.title);
        content.set(res.content);
        id.set(res.id);
    });

    view! {
        <article>
            <h2>{move||title.get()}</h2>
            <p>{move||content.get()}</p>
            <button>edit</button>
            <button on:click={move|ev: ev::MouseEvent|{
                let navigate = leptos_router::use_navigate();
                navigate("/", Default::default());
                }
            }>back</button>
        </article>
    }
}
