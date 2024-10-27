use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
      <Router>
        <h1>"Contact App"</h1>
        <nav>
          <h2>"Navigation"</h2>
          <a href="/">"Home"</a>
          <a href="/contacts">"Contacts"</a>
        </nav>        
        <main>
          <Routes>
            <Route path="/" view=|| view! {
              <h3>"Home"</h3>
            } />
            <Route path="/contacts" view=ContactList >
              <Route path=":id" view=ContactInfo>
                <Route path="" view=|| view! {
                  <div class="tab">
                    "(Contact Info)"
                  </div>
                }/>
                <Route path="conversations" view=|| view! {
                  <div class="tab">
                    "(Conversations)"
                  </div>
                }/>        
              </Route>
              <Route path="" view=|| view!{
                <div class="select-user" >
                  "Select a user to view contact info"
                </div>
              }/>
            </Route>
          </Routes>
        </main>
      </Router>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
          <div class="contact-list-contacts">
            <h3>"Contacts"</h3>
            <A href="giovanni">"Giovanni"</A>
            <A href="pietro">"Pietro"</A>
            <A href="jonas">"Jonas"</A>
          </div>
          <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let name = move || match id().as_str() {
        "giovanni" => "Giovanni",
        "pietro" => "Pietro",
        "jonas" => "Jonas",
        _ => "User not found.",
    };
    view! {
        <div class="contact-info">
          <h4>{name}</h4>
          <div class="tabs">
            <A href="" exact=true>"Contact Info"</A>
            <A href="conversations">"Conversations"</A>
          </div>
          <Outlet />
        </div>
    }
}

fn main() {
  leptos::mount_to_body(App)
}



