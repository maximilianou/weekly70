use leptos::*;
/*
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: i32,
}
*/
/*
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: RwSignal<i32>,
}
*/
#[derive(Debug, Clone)]
struct DatabaseEntry{
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {

  let (data, set_data) = create_signal(vec![
    DatabaseEntry {
        key: "Une".to_string(),
//        value: create_rw_signal(10),
        value: 100,
    },
    DatabaseEntry {
        key: "Due".to_string(),
//        value: create_rw_signal(20),
        value: 200,
    },
    DatabaseEntry {
        key: "Tre".to_string(),
//        value: create_rw_signal(30),
        value: 300,
    },
  ]);

  view! {
    <button on:click=move |_| {
        /*
      set_data.update(|data| {
        for row in data {
            row.value += 2;
        }
      });
            */
            /*
       data.with(|data|{
         for row in data {
            row.value.update(|value| *value *= 2);
         }
       });
       */
       set_data.update(|data| {
        for row in data {
            row.value += 2;
        }
      });

      logging::log!("{:?}", data.get());
    }>
      "Update Values"
    </button>

/*    <For each=data
//       key=|state| ( state.key.clone(), state.value ) 
         key=|state| state.key.clone()
         let:child
         >
         <p>{child.value}</p>
    </For>
*/
      <For
        each=move || data().into_iter().enumerate()
        key=|(_, state)| state.key.clone()
        children=move |(index, _)|{
          let value=create_memo( move |_| {
            data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
          });
          view! {
            <p>{value}</p>
          }
        }
      />
  }
}

fn main() {
    leptos::mount_to_body(App);
}
