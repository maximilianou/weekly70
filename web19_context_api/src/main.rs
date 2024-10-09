use leptos::{ev::MouseEvent, *};

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
fn App() -> impl IntoView {
  let (red, set_red) = create_signal(false);
  let (right, set_right) = create_signal(false);
  let (italics, set_italics) = create_signal(false);
  let (smallcaps, set_smallcaps) = create_signal(false);
  provide_context(SmallcapsContext(set_smallcaps));
  view! {
    <main>
      <p
        class:red=red
        class:right=right
        class:italics=italics
        class:smallcaps=smallcaps
      >
      "Writing down text into the paragraph"
      </p>
      <ButtonA setter=set_red />
      <ButtonB on_click=move |_| set_right.update( |value| *value = !*value ) />
      <ButtonC on:click=move |_| set_italics.update( |value| *value = !*value )  />
      <ButtonD />
    </main>
  }
}

#[component]
pub fn ButtonA(
    setter: WriteSignal<bool>,
) -> impl IntoView {
  view! {
    <button 
      on:click= move |_| setter.update( |value| *value = !*value ) 
    >
    "Toggle Red"
    </button>
  }
}

#[component]
pub fn ButtonB<F>(
    on_click: F,
) -> impl IntoView 
     where F: Fn(MouseEvent) + 'static,
{
  view! {
    <button on:click=on_click > "Toggle Right" </button>
  }  
}

#[component]
pub fn ButtonC() -> impl IntoView {
    view! {
        <button>"Toggle Italics"</button>
    }
}

#[component]
pub fn ButtonD() -> impl IntoView {
    let setter = use_context::<SmallcapsContext>().unwrap().0;
    view! {
        <button
          on:click=move |_| setter.update(|value| *value = !*value ) 
        >"Toggle Small Caps"</button>
    }
}

fn main() {
  leptos::mount_to_body(App);
}
