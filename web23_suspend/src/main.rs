use leptos::*;
use gloo_timers::future::TimeoutFuture;
async fn important_api_call(name: String) -> String {
  TimeoutFuture::new(1_000).await;
  name.to_ascii_uppercase()
}
#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Rene".to_string());
    let async_data = create_resource(
        name, |name| async move { important_api_call(name).await },
    ); 
    view! {
      <input
        on:input=move |ev| {
            set_name(event_target_value(&ev))
        }
        prop:value=name />
        <p><code>"name: "</code>{name}</p>
        <Suspense fallback=move || view! { <p>"Loading.."</p> } >
          <p>"Your shouting name is " 
            { move || async_data.get()}
          </p>
        </Suspense>
    }
}
fn main() {
  leptos::mount_to_body(App);
}
