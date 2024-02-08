use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Knowledge {
    pub id: String,
    pub name: Option<String>,
    pub audio: Option<String>,
    pub level: Option<u8>,
    pub sub_knowledge: Option<Vec<Knowledge>>
}

pub trait Levelable {
    fn level(&self) -> u8;
}

impl Levelable for Knowledge {
    fn level(&self) -> u8 {
        self.level.unwrap()
    }
}

pub trait HasAudio {
    fn audio(&self) -> Option<&str>;
    fn set_audio(&mut self, audio: Option<String>);
}

impl HasAudio for Knowledge {
    fn audio(&self) -> Option<&str> {
        self.audio.as_deref()
    }

    fn set_audio(&mut self, audio: Option<String>) {
        self.audio = audio;
    }
}