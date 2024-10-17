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

#[component]
fn CreateAnEffect() -> impl IntoView {
  let (first, set_first) = create_signal(String::new());
  let (last, set_last)   = create_signal(String::new());
  let (use_last, set_use_last) = create_signal(true);
  create_effect( move |_| {
    log(if use_last() {
      with!(|first, last| format("{first} {last}"))
    } else {
      first()
    })
  });
  view! {
    <h1>
      <code>"create effect"</code>
      "Version"
    </h1>
    <form>
      <label>
        "First Name"
        <input type="text" name="first" prop:value=first
          on:change=move |ev| set_first(event_target_value(&ev))  />
      </label>
      <label>
        "Last Name"
        <input type="text" name="last" prop:value=last 
          on:change=move |ev| set_last(event_target_value(&ev))  />
      </label>
      <label>
        "Show Last Name"
        <input type="checkbox" name="use_last" prop:checked=use_last
          on:change=move |ev| set_use_last(event_target_checked(&ev)) />
      </label>
    </form>
  }
}

#[compnent]
fn ManualVersion() -> impl IntoView {
    let first    = create_node_ref::<Input>();
    let last     = create_node_ref::<Input>();
    let use_last = create_node_ref::<Input>();
    let mut prev_name = String::new();
    let on_change = move |_| {
      log("      listener");
      let first    = first.get().unwrap();
      let last     = last.get().unwrap();
      let use_last = use_last.get().unwrap();
      let this_one = if use_last.checked() {
        format!("{} {}", first.value(), last.value())
      } else {
        first.value()
      };
      if this_one != prev_name {
        log(&this_one);
        prev_name = this_one;
      }
    };
    view! {
      <h1>
        "Manual Version"
      </h1>
      <form on:change=on_change>
        <label>"First Name" 
        <input />
        </label>
      </form>
    }
}

fn main() {
  leptos::mount_to_body(App);
}



