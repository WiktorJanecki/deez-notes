use gloo_storage::Storage;
use leptos::*;

use crate::{
    login::AUTH_TOKEN,
    note::{Note, NoteTitle},
    API_PATH,
};

#[component]
pub fn Home() -> impl IntoView {
    let login_signal = expect_context::<RwSignal<bool>>();
    let token_signal: RwSignal<Option<String>> = create_rw_signal(None);
    let notes_signal: RwSignal<Vec<Note>> = create_rw_signal(vec![]);
    create_effect(move |_| {
        let token = gloo_storage::LocalStorage::get::<String>(AUTH_TOKEN).ok();
        token_signal.set(token.clone());
        if token.is_some() {
            login_signal.set(true);
        }
    });

    spawn_local(async move {
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{API_PATH}/notes"))
            .fetch_credentials_include()
            .send()
            .await
            .expect("API ERROR");
        let notes = res.json::<Vec<Note>>().await.expect("API ERR");
        notes_signal.set(notes);
    });

    move || match login_signal.get() {
        true => view! {<>
            <h3>"Notes: "</h3>
            <article>
                {move||{
                    let notes = notes_signal.get();
                    notes.iter().map(|note|{
                        view!{<NoteTitle id=note.id title=note.title.clone() />}
                    }).collect::<Vec<_>>()
                }}
                <button>Add new note</button>
            </article>
        </>},
        false => view! {<><h3>"Must be logged in"</h3></>},
    }
}
