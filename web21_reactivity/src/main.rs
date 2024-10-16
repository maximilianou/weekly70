use leptos::html::Input;
use leptos::*;

#[derive(Copy, Clone)]
struct LogContext( RwSignal<Vec<String>> );

#[component]
fn App() -> impl IntoView {
  let log = create_rw_signal::<Vec<String>>(vec![]);
  let logged = move || log().join("\n");
  provide_context(LogContext(log));
  view! {
    <CreateAnEffect/>
    <pre>{logged}</pre>
  }
}

fn main() {
    println!("Hello, world!");
}



