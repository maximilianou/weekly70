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

