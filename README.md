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


cargo add leptos --features=csr,nigthly

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

#TODO: here

```







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

