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

    // run at start
    create_effect(move |_| {
        let token = gloo_storage::LocalStorage::get::<String>(AUTH_TOKEN).ok();
        // TODO: check on server if token is still valid
        if token.is_some() {
            login_signal.set(true);
        }
    });

    move || match login_signal.get() {
        true => view! {<>
            <Await future=fetch_notes let:data>
            <h3>"Notes: "</h3>
            <article>
                {
                    data.iter()
                    .map(|note| view!{<NoteTitle id=note.id title=note.title.clone()/>})
                    .collect::<Vec<_>>()
                }
                <button>Add new note</button>
            </article>
            </Await>
        </>},
        false => view! {<><h3>"Must be logged in"</h3></>},
    }
}

async fn fetch_notes() -> Vec<Note> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{API_PATH}/notes"))
        .fetch_credentials_include()
        .send()
        .await
        .expect("API ERROR");
    res.json::<Vec<Note>>().await.expect("API ERR")
}
