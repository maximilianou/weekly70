use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;

async fn add_todo(text: &str) -> Uuid{
    _ = text;
    TimeoutFuture::new(1_000).await;
    Uuid::new_v4()
}
#[component]
fn App() -> impl IntoView {
  let add_todo = create_action( |input: &String| {
    let input = input.to_owned();
    async move { add_todo(&input).await }
  });
  let submitted = add_todo.input();
  let pending   = add_todo.pending();
  let todo_id   = add_todo.value();
  let input_ref = create_node_ref::<Input>();
  view! {
    <form
      on:submit= move |ev| {
        ev.prevent_default();
        let input = input_ref.get().expect("input to exist");
        add_todo.dispatch( input.value() );
      }
    >
      <label>"What do you need to do ?"
        <input type="text" node_ref=input_ref />
      </label>
      <button type="submit" >"Add Todo"</button>
    </form>
    <p>{move || pending().then(|| "Loading..") }</p>
    <p>"Submitted"
      <code>{move || format!("{:#?}", submitted())}</code>
    </p>
    <p>"Pending"
      <code>{move || format!("{:#?}", pending())}</code>
    </p>
    <p>"Todo Id"
      <code>{move || format!("{:#?}", todo_id())}</code>
    </p>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
