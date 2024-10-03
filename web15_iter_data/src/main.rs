use leptos::*;

#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {



    view! {

    }
}

fn main() {
    leptos::mount_to_body(App);
}
