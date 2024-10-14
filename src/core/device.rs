pub trait Device {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_state(&self) -> HashMap<String, String>;
    fn set_state(&mut self, state: HashMap<String, String>);
    fn send_cmd(&mut self, command: &str, parameters: Option<HashMap<String, String>>);
}

/**
 * Common Device Config
**/
pub struct Config {
    id: String,
    name: String,
    device_type: Type,
    address: String,
    handler dyn ProtocolHandler
}

/**
 * Generic Device Types
**/
pub enum Type {
    Sensor,
    Actor,
    Switch,
    Controller,
    Cat
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
    Purring
}

/**
 * Core Protocol Handler
**/
pub trait ProtocolHandler {
    
    fn protocol_name(&self) -> &str;
    fn create_device(&mut self, config: &DeviceConfig) -> Result<Box<dyn Device>, String>;
    fn send_cmd(&mut self, device: &mut dyn Device, cmd: &str, params: Option<HashMap<String, String>>;
    fn initialize(&mut self) -> Result<(), String>
}

/**
 * Device Actions
**/
pub enum Action {
    TurnOn,
    TurnOff,
    Set(HashMap<String, String>),
    reset(String),
}

pub trait Action {
    fn execute(&self, device: &mut dyn Device) -> Result<(), String>
}

pub struct DeviceFactory {
    registrar: Arc<RwLock<HashMap<String, Box<dyn Fn(DeviceConfig) -> Box<dyn Device>>>>>
}

impl DeviceFactory {
 pub fn new() -> Self {
     DeviceFactory {
         registrar: Arc::new(RwLock::new(HashMap::new())),
     }
 }

}
