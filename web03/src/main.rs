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

