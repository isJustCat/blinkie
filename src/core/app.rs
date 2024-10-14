use super::device::{Config, Device, ProtocolRegistry};
use std::sync::{Arc, RwLock};

pub enum AppState {
    STARTING,
    OK,
    ERROR,
    UNKNOWN,
}

pub struct App {
    pub devices: Arc<RwLock<Vec<Box<dyn Device>>>>,
    pub protocol_registry: Arc<RwLock<ProtocolRegistry>>,
    pub state: AppState,
}
