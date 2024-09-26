use leptos::*;
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn ProgressBar(
  #[prop(into)]
  progressValue: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress max="100" value=progressValue />
    }
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count( count() + 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
        <progress max="100" value=double_count />
        <p>"Double Count: " {double_count}</p>
        <ProgressBar progressValue=count />
    }
}


