use leptos::*;
fn main() {
    leptos::mount_to_body( || view! { <App/> }  );
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let values = vec![0, 1, 2];

    let length = 5;
    let counters = (1..=length).map( |idx| create_signal(idx) );

    let counter_buttons = counters
      .map( |( count, set_count )|{
        view! {
          <li>
            <button
              on:click=move |_| set_count.update(|n| *n += 3)
            >{count}
            </button>
          </li>
        }
      }).collect_view();

    view! {
        <p>{values.clone()}</p>
        <ul>
          {values.into_iter()
            .map(|n| view! {<li>{n}

                <button 
                on:click=move |_| {
                  set_count( count() + 1 );
                }
              >
              "Click Here:" {move || count()} 
              </button>
                        

                </li>})
            .collect::<Vec<_>>()
          }
      </ul>

      <ul>{counter_buttons}</ul>
    }

}


