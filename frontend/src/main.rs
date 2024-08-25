extern crate dotenv;
use edit::EditComponent;
use home::Home;
use lazy_static::lazy_static;
use leptos::*;
use leptos_router::{use_location, Route, Router, Routes};
use login::Login;
use nav::Navigation;
use note::NoteComponent;
use register::Register;

mod edit;
mod home;
mod login;
mod nav;
mod note;
mod register;

lazy_static! {
    pub static ref API_PATH: &'static str = std::env!("API_PATH");
}

#[derive(Copy, Clone)]
pub struct Context {
    pub login_signal: RwSignal<bool>,
    pub error_signal: RwSignal<String>,
}

#[component]
fn App() -> impl IntoView {
    let login_signal = create_rw_signal(false);
    let error_signal = create_rw_signal(String::from(""));

    provide_context(Context {
        login_signal,
        error_signal,
    });

    create_effect(move |_| {
        let _ = use_location().pathname.get(); // makes effect run every path change
        error_signal.set("".to_owned());
    });

    view! {
        <Navigation />
        <main>
            <Router>
               <Routes>
                  <Route path="/" view=Home/>
                  <Route path="/login" view=Login />
                  <Route path="/register" view=Register />
                  <Route path="/note/:id" view=NoteComponent />
                  <Route path="/edit/:id" view=EditComponent />
                  <Route path="*any" view= || view!{<h1>"Not found!"</h1>}/>
               </Routes>
            </Router>
            <p class="err">{move||error_signal.get()}</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    println!("Hello, world!");
    mount_to_body(|| view! { <App /> })
}
