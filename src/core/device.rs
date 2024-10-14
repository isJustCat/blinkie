/**
 * Device Interface
***/
pub trait Device {
    pub fn get_id(&self) -> &str;
    pub fn get_name(&self) -> &str;
    pub fn get_state(&self) -> HashMap<String, String>;
    pub fn set_state(&mut self, state: HashMap<String, String>);
    pub fn send_cmd(&mut self, command: &str, parameters: Option<HashMap<String, String>>);
}

/**
 * Common Device Config
**/
pub struct Config {
    pub id: String,
    pub name: String,
    pub device_type: Type,
    pub connection_details: HashMap<String, String>,
    pub supported_protocols: Vec<String>,
    pub preferred_handler: Option<String>,
}

/**
 * Generic Device Types
**/
pub enum Type {
    Sensor,
    Actor,
    Switch,
    Controller,
    Cat,
}

/**
 * Device State
**/
pub enum State {
    On,
    Off,
    Running,
    Finished,
    Error,
    Unknown,
    Purring,
}

pub struct ProtocolRegistry {
    pub handlers: Arc<RwLock<HashMap<String, Arc<dyn<ProtocolHandler>>>>>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        ProtocolRegistry {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, handler: Arc<dyn<ProtocolHandler>>) {
        let mut handlers = self.handlers.write().unwrap();
        for protocol in handler.supported_protocols() {
            handlers.entry(protocol.clone()).push(Arc::clone(&handler));
        }

        for list in handlers.vals_mut() {
            list.sort_by(|a, b| b.priority().cmp(&a.priority()));
        }
    }

    pub fn get_handlers(&self, protocol: &str) -> Option<Vec<Arc<dyn<ProtocolHandler>>>> {
        let handlers = self.handlers.read().unwrap();
        handlers.get(protocol).cloned()
    }
}

/**
 * Protocol Handler Interface
**/
pub trait ProtocolHandler: Send + Sync {
    fn name(&self) -> String;
    fn priority(&self) -> u8 {
        0
    }
    fn supported_protocols(&self) -> Vec<String>;
    fn create_device(&mut self, config: &DeviceConfig) -> Result<Box<dyn<Device>>, String>;
    fn send_cmd(
        &mut self,
        device: &mut dyn Device,
        cmd: &str,
        params: Option<HashMap<String, String>>,
    ) -> Result<(), String>;
    fn initialize(&mut self) -> Result<(), String>;
}

/**
 * Device Actions
**/
pub enum Action {
    TurnOn,
    TurnOff,
    Set(HashMap<String, String>),
    Reset(String),
}

/**
 * Action Interface
**/
pub trait Action {
    pub fn execute(&self, device: &mut dyn Device) -> Result<(), String>;
}

pub struct DeviceFactory {
    pub registrar: Arc<RwLock<HashMap<String, Box<dyn Fn(DeviceConfig) -> Box<dyn Device>>>>>,
}

impl DeviceFactory {
    pub fn new() -> Self {
        DeviceFactory {
            registrar: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&mut self, config: DeviceConfig) -> Result<Box<dyn Device>, String> {
        for protocol in &config.supported_protocols {
            if let Some(handlers) = self.protocol_registry.get_handlers(protocol) {
                if let Some(preferred) = &config.preferred_handler {
                    if let Some(handler) = handlers.into_iter().find(|h| h.name() == *preferred) {
                        return handler.connect_device(&config.connection_details);
                    } else {
                        return Err(format!(
                            "Preferred handler '{}' not found for protocol: {}",
                            preferred, protocol
                        ));
                    }
                } else {
                    let handler = &handlers[0]; 
                    return handler.connect_device(&config.connection_details);
                }
            }
        }
        Err(format!(
            "No compatible protocol handler found for device: {}",
            config.name
        ))
    }
}
