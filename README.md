# weekly70 - rust - 2024

-----------

## leptos.dev

-------------

https://book.leptos.dev/getting_started/index.html

```sh
cargo install trunk

cargo init web07_leptos

cd web07_letpos

# install global 
rustup toolchain install nightly
rustup default nightly

# install in the project only
rustup toolchain install nightly
cd web07_leptos
rustup override set nightly

# install wasm32 to compile and run webassembly in the browser
rustup target add wasm32-unknown-unknown


cargo add leptos --features=csr,nightly

```
```html
<!DOCTYPE html>
<html>
    <head><title></title></head>
    <body></body>
</html>
```
```rust
use leptos::*;
fn main() {
    mount_to_body( || view! { <><p>"Hi, here we are!"</p><ul><li>"Rust"</li><li>"Web"</li></ul></> } )
}
```
```sh


cargo add console_error_panic_hook

console_error_panic_hook::set_once();


https://book.leptos.dev/view/index.html

cargo install trunk
cargo install cargo-generate

##
## on debian - to have SSL libraries install librust-openssl-dev
##
root@srv04:~# apt install librust-openssl-dev

cargo install cargo-generate



cargo generate --git https://github.com/leptos-community/start-csr

trunk serve --port 3000 --open



```

### Rust leptos.dev web components 


```sh

https://book.leptos.dev/view/01_basic_component.html


```
```rust
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}

```

```sh
cargo init web09_components
cd web09_components
cargo add leptos --features=csr,nightly

touch index.html
```

```html
<!DOCTYPE html>
<html>
    <head><title></title></head>
    <body></body>
</html>
```

```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count(3);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
    }
}

```

```sh

https://book.leptos.dev/view/02_dynamic_attributes.html

```

```html
<!DOCTYPE html>
<html>
    <head><title></title>
        <style>
            .red {
                color: red;
            }
        </style>
    </head>
    <body></body>
</html>

```

```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body( || view! { <App />}    );
}
#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  view! {
    <button on:click=move |_| { set_count.update( |n| *n += 1); } 
    class:red=move || count() % 2 == 1
    >
      "Click me:" {move || count()}
    </button>
  }  
}
```



```rust
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
use leptos::SignalUpdate;
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body( || view! { <App />}    );
}
#[component]
fn App() -> impl IntoView {
  let (count, set_count) = create_signal(0);
  let (x, _set_x) = create_signal(0);
  view! {
    <button on:click=move |_| { set_count.update( |n| *n += 1); } 
    class:red=move || count() % 2 == 1
    >
      "Click me:" {move || count()}
    </button>
    <button on:click=move |_| { _set_x.update( |n| *n += 10); } 
    style="position: absolute"
    style:left=move || format!("{}px", x() + 100)
    style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
    style:max-width="400px"
    style=("--columns", x)

    >
      "Move this:" 
    </button>
  }  
}
```
```sh

https://book.leptos.dev/view/03_components.html


```
```rust
use leptos::*;
use leptos::view;
use leptos::IntoView;
use leptos::create_signal;
use leptos::component;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn ProgressBar(
  #[prop(into)]
  progressValue: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress max="100" value=progressValue />
    }
}


#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                set_count( count() + 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {move || count()}
        </button>
        <progress max="100" value=double_count />
        <p>"Double Count: " {double_count}</p>
        <ProgressBar progressValue=count />
    }
}

```



```sh
https://book.leptos.dev/view/04_iteration.html

```

```sh
Static List
```

```rust
use leptos::*;
#[component]
fn App() -> impl IntoView {
    view!{
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if your list is static"</p>
        <StaticList length=5 />
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the list in your list will change"</p>
        <DynamicList initial_length=7 />
    }
}
#[component]
fn StaticList( length: usize) -> impl IntoView {
    let counters = (1..=length).map( |idx| create_signal(idx) );
    let counter_buttons = counters
          .map( |(count, set_count)| {
            view!{
              <li>
                <button 
                on:click=move |_| set_count.update( |n| *n += 1 )
                >
                  {count}
                </button>
              </li>
            }
          }).collect::<Vec<_>>();
  view! {
    <ul>
      {counter_buttons}
    </ul>
  }
}
#[component]
fn DynamicList( initial_length: usize ) -> impl IntoView {
  view! {
    <ul>
    <li>{initial_length}</li>
    </ul>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
```


```sh
Dynamic List with For
```

```rust
use leptos::*;
#[component]
fn App() -> impl IntoView {
    view!{
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if your list is static"</p>
        <StaticList length=5 />
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the list in your list will change"</p>
        <DynamicList initial_length=7 />
    }
}
#[component]
fn StaticList( length: usize) -> impl IntoView {
    let counters = (1..=length).map( |idx| create_signal(idx) );
    let counter_buttons = counters
          .map( |(count, set_count)| {
            view!{
              <li>
                <button 
                on:click=move |_| set_count.update( |n| *n += 1 )
                >
                  {count}
                </button>
              </li>
            }
          }).collect::<Vec<_>>();
  view! {
    <ul>
      {counter_buttons}
    </ul>
  }
}
#[component]
fn DynamicList( initial_length: usize ) -> impl IntoView {
  let mut next_counter_id = initial_length;
  let initial_counters = (0..initial_length)
    .map(|id| (id, create_signal(id+1)))
    .collect::<Vec<_>>();
  let (counters, set_counters) = create_signal(initial_counters);
  let add_counter = move |_| {
    let sig = create_signal(next_counter_id + 1);
    set_counters.update( move |counters| {
      counters.push((next_counter_id, sig))
    });
    next_counter_id +=1;
  };
  view! {
    <div>
      <button on:click=add_counter>
        "Add Counter"
      </button>
      <ul>
        <For 
          each=counters
          key=|counter| counter.0
          children=move |(id, (count, set_count))|{
            view! {
                <li>
                  <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                  >
                    {count}
                  </button>
                  <button
                    on:click=move |_| {
                      set_counters.update(|counters| {
                        counters.retain(|(counter_id, _)| counter_id != &id)
                      });
                    }
                  >
                    "Remove"
                  </button>

                </li>
            }
          }
        />
      </ul>
    </div>
  }
}

fn main() {
  leptos::mount_to_body(App)
}
```


```sh
Iteration and data structure in the frontend
```



```rust

```



```sh
## TODO: here

```































































----------------------------------------------
----------------------------------------------
----------------------------------------------
## Yew
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

-----------


```rust
use yew::{prelude::*, virtual_dom::VNode};
#[function_component(App)]
fn app() -> Html {

  struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,  
  }  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 1".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 2".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 3".to_string(),
    },
  ];
  let videos: VNode = videos.iter().map(|video: &Video| html!{ 
    <>
    <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
    <img url={video.url.clone()} />    
    </>
  }).collect::<Html>();

  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        {videos}
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------


```rust
use yew::{prelude::*};
#[derive(Clone, PartialEq)]
struct Video {
  id: usize,
  title: String,
  speaker: String,
  url: String,  
}
#[derive(Properties, PartialEq)]
struct VideosListProps {
  videos: Vec<Video>,
}
#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
  videos.iter().map(|video: &Video| html!{ 
    <>
    <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
    <img url={video.url.clone()} />    
    </>
  }).collect()
}
#[function_component(App)]
fn app() -> Html {
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 1".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 2".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "https://via.placeholder.com/640x360.png?text=Homelab Day 3".to_string(),
    },
  ];  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------

#### Modules

```rust
use yew::{prelude::*};
use crate::video::*;
mod video {
  use yew::{prelude::*};
  #[derive(Clone, PartialEq)]
  pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,  
  }  
  #[derive(Properties, PartialEq)]
  pub struct VideosListProps {
    pub videos: Vec<Video>,
  }
  #[function_component(VideosList)]
  pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video: &Video| html!{ 
      <>
      <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
      <img url={video.url.clone()} />    
      </>
    }).collect()
  }
}
#[function_component(App)]
fn app() -> Html {  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v01homelab01proxmox.jpg".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v02homelab02.webp".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v03homelab03gitopskubernetes.webp".to_string(),
    },
  ];  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
```

-----------

#### Modules in different files

- src/main.rs
```rust
use yew::{prelude::*};
mod video;
use video::video::*;
#[function_component(App)]
fn app() -> Html {  
  let videos: Vec<Video> = vec![
    Video {
      id: 1,
      title: "Homelab Day 1: proxmox".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v01homelab01proxmox.jpg".to_string(),
    },
    Video {
      id: 2,
      title: "Homelab Day 2: ".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v02homelab02.webp".to_string(),
    },
    Video {
      id: 3,
      title: "Homelab Day 3: GitOps way of kubernetes".to_string(),
      speaker: "Maximiliano Usich".to_string(),
      url: "img/v03homelab03gitopskubernetes.webp".to_string(),
    },
  ];
  
  html! {
    <>
      <h1>{ "Video Tutorial Explorer! " }</h1>
      <div>
        <VideosList videos={ videos } />
      </div>
     </>
  }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

- src/video/mod.rs
```rust
pub mod video {
    use yew::{prelude::*};  
    #[derive(Clone, PartialEq)]
    pub struct Video {
      pub id: usize,
      pub title: String,
      pub speaker: String,
      pub url: String,  
    }
    #[derive(Properties, PartialEq)]
    pub struct VideosListProps {
      pub videos: Vec<Video>,
    }    
    #[function_component(VideosList)]
    pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
      videos.iter().map(|video: &Video| html!{ 
        <>
        <p key={ video.id }>{format!(" {}: {} ", video.speaker, video.title)}</p>
        <img url={video.url.clone()} />    
        </>
      }).collect()
    }
  }
```

```sh
trunk serve
```

-----------

#### Modules 

