
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
  