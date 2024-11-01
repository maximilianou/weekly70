use leptos::*;
use leptos_router::*;
#[component]
fn App() -> impl IntoView {
  view! {
    <Router>
      <main style="background-color: lightblue; padding: 10px;">
        <Routes>
          <Route path="" view=HomePage />
        </Routes>
      </main>
    </Router>
  }
}
#[component]
fn HomePage() -> impl IntoView {
  let files = ["a.txt", "b.txt", "c.txt"];
  let labels = files.iter().copied().map(Into::into).collect();
  let tabs = move || {
    files
       .into_iter()
       .enumerate()
       .map( | (index, filename) | {
         let content = std::fs::read_to_string(filename).unwrap();
         view! {
            <Tab index>
              <div style="background-color: lightblue ;  padding:10px" >
                <h2>{filename.to_string()}</h2>
                <p>{content}</p>
              </div>
            </Tab>
         }
       } ).collect_view()
  };
  view! {
    <h1>"Welcome to Leptos!"</h1>
    <p>"Click any of the tabs below to read a recipe"</p>
    <Tab labels>
      <div>{tabs()}</div>
    </Tab>
  }
}
#[island]
fn Tabs(labels: Vec<String>, children: Children) -> IntoView {
  let (selected, set_selected) = create_signal(0);
  provide_context(selected);
  let buttons = labels
    .into_iter()
    .enumerate()
    .map( | (index, label) | {
      view! {
        <button on:click=move |_| set_selected(index)>
          {label}
        </button>
      }
    }).collect_view();
    view! {
        <div style="display: flex; width: 100%; justify-content: space-around; background-color: lightgreen; padding: 10px;">
          {button}
        </div>
        {children()}
    }
}
#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
  let selected = expect_content::<ReadSignal<usize>>();
  view! {
    <div
      style:background-color="lightgreen"
      style:padding="10px"
      style:display=move || if selected() == index {
        "block"
      }else{
        "none"
      }
    >
      {children()}
    </div>
  }
}

fn main() {
  leptos::mount_to_body(App);
}
