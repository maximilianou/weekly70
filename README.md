# weekly70 - rust - 2024


```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <p>{ "Homelab Day 1: " }</p>
        <p>{ "Homelab Day 2: " }</p>
        <p>{ "Homelab Day 3: GitOps way of kubernetes " }</p>
      </div>
      <div>
        <h3>{ "Homelab Day 3: GitOps way of kubernetes " }</h3>
        <img src="https://via.placeholder.com/640x360.png?text=Homelab Day 3: GitOps way of kubernetes" alt="video thumbnail" />
      </div>
    </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```