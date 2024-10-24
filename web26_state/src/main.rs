use leptos::*;

#[component]
fn Option2() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    provide_context(count);
    view! {
      <h1>"Option 2 : Passing Signals"</h1>
      <SetterButton set_count />
      <div style="display:flex">
        <FancyMath/>
        <ListItems/>
      </div>
    }
}
#[component]
fn SetterButton(set_count: WriteSignal<u32>) -> impl IntoView {
  view! {
    <div class="provider red" style="background-color:red;">
    <button on:click=move |_| set_count.update(|count| *count += 1) >
      "Increment Global Count"
    </button>
    </div>
  }
}
#[component]
fn FancyMath() -> impl IntoView {
    let count = use_context::<ReadSignal<u32>>()
    .expect("there to be a count signal provided");
    let is_even = move || count() & 1 == 0;
    view! {
        <div class="consumer blue">
          "the number "
          <strong>{count}</strong>
          {move || if is_even() {
            " is "
          }else{
            " is not "
          }}
          " even."
        </div>
    }
}
#[component]
fn ListItems() -> impl IntoView {
    let count = use_context::<ReadSignal<u32>>().expect("there to be a count signal provided");
    let squares = move || {
        (0..count())
          .map(|n| view! {<li>{n}<sup>"2"</sup>" is " {n*n}</li>})
          .collect::<Vec<_>>()
    };
    view! {
        <div class="consumer green">
          <ul>{squares}</ul>
        </div>
    }
}
#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    count: u32,
    name: String,
}
#[component]
fn Option3() -> impl IntoView {
  let state = create_rw_signal(GlobalState::default());
  provide_context(state);
  view! {
    <h1>"Option 3: Passing signals"</h1>
    <div class="red consumer" style="with: 10%;background-color:red;">
      <h2>"Current Glocal State"</h2>
      <pre>
        {move || {
            format!("{:#?}", state.get())
        }}
      </pre>
    </div>
    <div style="display: flex">
      <GlobalStateCounter />
      <GlobalStateInput />
    </div>
  }  
}
#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");
    let (count, set_count) = create_slice(
        state,
        |state| state.count,
        |state, n| state.count = n,
    );
    view! {
      <div class="consume blue" style="background-color: blue;">
        <button
          on:click=move |_| {
            set_count( count() + 1 );
          }
        >"Incremental Global Count"</button>
        <br/>
        <span>"Count is: "{count}</span>
      </div>
    }
}
#[component]
fn GlobalStateInput() -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>().expect("state to have been provided");
    let (name, set_name) = create_slice(
        state,
        |state| state.name.clone(),
        |state, n| state.name = n,
    );
    view! {
      <div class="consumer green" style="background-color:green;">
        <input 
          type="text"
          prop:value=name
          on:input=move |ev| {
            set_name(event_target_value(&ev));
          }
        />
        <br/>
        <span>"Name is: " {name}</span>
      </div>
    }
}
fn main() {
    leptos::mount_to_body( || view! { <Option2/><Option3/> } )
}
