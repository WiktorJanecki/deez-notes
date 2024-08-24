use gloo_storage::{LocalStorage, Storage};
use leptos::*;

use crate::{login::AUTH_TOKEN, Context};

#[component]
pub fn Navigation() -> impl IntoView {
    let ctx = expect_context::<Context>();
    let login_signal = ctx.login_signal;
    let logout = move |_| {
        LocalStorage::delete(AUTH_TOKEN);
        login_signal.set(false);
    };
    let log_symbol = move || {
        if login_signal.get() {
            view! {<a on:click=logout href="/">"Logout"</a>}
        } else {
            view! {<a href="/login">"Login"</a>}
        }
    };
    view! {
        <nav class="">
           <a href="/">"Home"</a>
           {log_symbol}
        </nav>
    }
}
