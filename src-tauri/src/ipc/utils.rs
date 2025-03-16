use std::{collections::HashMap, pin::Pin, sync::Arc};

// use reqwest::header; 

pub struct Event {
    pub identifier: String, 
    pub channels: Vec<String>,
    pub content: Option<String>
}

impl Event {
    pub fn new<S: ToString + Sized>(identifier: &S, channels: &Vec<S>, content: &Option<S>) -> Self {
        let mut stringified_channels: Vec<String> = Vec::new();
        for channel in channels {
            stringified_channels.push(channel.to_string());
        }
        let stringified_content = content.as_ref().map(|s| s.to_string());
        Self {
            identifier: identifier.to_string(),
            channels: stringified_channels,
            content: stringified_content
        }
    }
}

type EmitterMap = HashMap<String, Vec<Arc<dyn Fn(String, Option<String>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>>;

pub struct Emitter {
    map: EmitterMap
}

impl Emitter {
    pub fn on<S: ToString + ?Sized, F: 'static + Fn(String, Option<String>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>(&'static mut self, channel: &S, operation: F) {
        let channel_string = channel.to_string();
        let map_value = self.map.get_mut(&channel_string).unwrap();
        map_value.push(Arc::new(operation));
    }

    pub fn emit(&'static self, event: &Event) {
        for channel in event.channels.clone().into_iter() {
            for operations in self.map.get(&channel).unwrap() {
                tokio::spawn(
                    (operations.clone())(event.identifier.clone(), event.content.clone())
                );
            }
        }
    }    
} 