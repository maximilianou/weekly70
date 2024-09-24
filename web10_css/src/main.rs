use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body( || view! { <App />}    );
}
#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let (x, _set_x) = create_signal(0);
  view! {
    <button on:click=move |_| { set_count.update( |n| *n += 1); } 
    class:red=move || count() % 2 == 1
    >
      "Click me:" {move || count()}
    </button>
    <button on:click=move |_| { _set_x.update( |n| *n += 10); } 
    style="position: absolute"
    style:left=move || format!("{}px", x() + 100)
    style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
    style:max-width="400px"
    style=("--columns", x)

    >
      "Move this:" 
    </button>
  }  
}