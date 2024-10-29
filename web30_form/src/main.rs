use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1><code>" <Form /> "</code></h1>
            <main>
              <Routes>
               <Route path="" view=FormExample />
              </Routes>
            </main>
        </Router>
    }
}
#[component]
fn FormExample() -> impl IntoView {
    let query  = use_query_map();
    let name   = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();
    view! {
        <table>
          <tr>
            <td><code>"name"</code></td>
            <td>{name}</td>
          </tr>
          <tr>
            <td><code>"number"</code></td>
            <td>{number}</td>
          </tr>
          <tr>
            <td><code>"select"</code></td>
            <td>{select}</td>
          </tr>
        </table>
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="" >
        </Form>
    }
}
fn main() {
    leptos::move_to_body(App);
}