use edit::EditComponent;
use home::Home;
use leptos::*;
use leptos_router::{Route, Router, Routes};
use login::Login;
use note::NoteComponent;

mod edit;
mod home;
mod login;
mod note;

const API_PATH: &str = "http://localhost:3000/api";

#[component]
fn App() -> impl IntoView {
    let login_signal = create_rw_signal(false);
    provide_context(login_signal);
    view! {
        <main>
            <nav class="">
               <a href="/">"Home"</a>
               <a href="/login">"Login"</a>
            </nav>
            <Router>
               <Routes>
                  <Route path="/" view=Home/>
                  <Route path="/login" view=Login />
                  <Route path="/note/:id" view=NoteComponent />
                  <Route path="/edit/:id" view=EditComponent />
                  <Route path="*any" view= || view!{<h1>"Not found!"</h1>}/>
               </Routes>
            </Router>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    println!("Hello, world!");
    mount_to_body(|| view! { <App /> })
}
