use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None }; 
    view! {
      <h1>"Control Flow"</h1>
      <button on:click=move |_| set_value.update(|n| *n += 1 ) >
      " +1 "
      </button>
      <p>"Value is : " { value }</p>
      <hr />
      <h2><code>"Option<T>"</code></h2>

      <p>{odd_text}</p>
      <p>{ move || odd_text().map(|text| text.len()) }</p>
      <p>"Conditional Logic"</p>
      <p>{ move || if is_odd() {
            "Odd!"
          }else{
            "Even!"
          }}</p>
      <p class:hidden=is_odd>"Appears if Even!"</p>
      <Show when=is_odd
            fallback=|| view! { <p>"Even Steven"</p>}>
        <p>"Oddment"</p>
      </Show>
      { move || is_odd().then( || view! { <p>"Oddity!"</p>} ) }
      <p>"Converting Between Types"</p>
      {
        move || match is_odd() {
            true if value() == 1 => {
                view! { <pre>"One"</pre> }.into_any()
            },
            false if value() == 2 => {
              view! { <p>"Two"</p> }.into_any()
            }
            _ => view! { <textarea>{value()}</textarea> }.into_any()
        }
      }
    }
}
fn main() {
  leptos::mount_to_body(App);
}
