use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count.update( |n| *n += 1 );
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}
