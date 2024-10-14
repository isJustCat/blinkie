use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/**
 * Device Interface
 * Represents a generic Device
 */
pub trait Device {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_state(&self) -> HashMap<String, String>;
    fn set_state(&mut self, state: HashMap<String, String>);
    fn send_cmd(&mut self, command: &str, parameters: Option<HashMap<String, String>>);
}

/**
 * Common Device Config
 * Holds metadata and connection information for devices.
 */
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub id: String,
    pub name: String,
    pub device_type: Type,
    pub connection_details: HashMap<String, String>,
    pub supported_protocols: Vec<String>,
    pub preferred_handler: Option<String>,
}

/**
 * Device Types
 *
 */
#[derive(Serialize, Deserialize)]
pub enum Type {
    Sensor,
    Actor,
    Switch,
    Controller,
    Cat,
}

/**
 * Device States
 */
#[derive(Serialize, Deserialize)]
pub enum State {
    ON,
    OFF,
    RUNNING,
    FINISHED,
    ERROR,
    UNKNOWN,
    PURRING,
}

/**
 * ProtocolRegistry
 * Manages the registration and retrieval of protocol handlers.
 */
pub struct ProtocolRegistry {
    pub handlers: Arc<RwLock<HashMap<String, Vec<Arc<RwLock<dyn ProtocolHandler>>>>>>,
}

impl ProtocolRegistry {
    pub fn new() -> Self {
        ProtocolRegistry {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers a protocol handler, associating it with its supported protocols.
    pub fn register(&self, handler: Arc<RwLock<dyn ProtocolHandler>>) {
        let mut handlers = self.handlers.write().unwrap();

        // Insert handler into all supported protocol entries
        for protocol in handler.read().unwrap().supported_protocols() {
            handlers
                .entry(protocol.clone())
                .or_insert_with(Vec::new)
                .push(Arc::clone(&handler));
        }

        // Sort handlers by priority in descending order
        for list in handlers.values_mut() {
            list.sort_by(|a, b| {
                let a_priority = a.read().unwrap().priority();
                let b_priority = b.read().unwrap().priority();
                b_priority.cmp(&a_priority)
            });
        }
    }

    /// Retrieves the list of handlers for a given protocol, if any exist.
    pub fn get_handlers(&self, protocol: &str) -> Option<Vec<Arc<RwLock<dyn ProtocolHandler>>>> {
        let handlers = self.handlers.read().unwrap();
        handlers.get(protocol).cloned()
    }
}

/**
 * ProtocolHandler Interface
 * Defines the behavior of a protocol handler that can manage devices.
 */
pub trait ProtocolHandler: Send + Sync {
    fn name(&self) -> String;
    fn priority(&self) -> u8 {
        0 // Default priority is 0
    }
    fn supported_protocols(&self) -> Vec<String>;
    fn create_device(&mut self, config: &Config) -> Result<Box<dyn Device>, String>;
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
 */
#[derive(Serialize, Deserialize)] // Added Deserialize for this enum
pub enum Action {
    TurnOn,
    TurnOff,
    Set(HashMap<String, String>),
    Reset(String),
}

/**
 * ActionExecutor Interface
 * Provides an abstraction for executing an action on a device.
 */
pub trait Executor {
    fn execute(&self, device: &mut dyn Device) -> Result<(), String>;
}

/**
 * DeviceFactory
 * Responsible for creating devices using registered handlers.
 */
pub struct DeviceFactory {
    pub protocol_registry: Arc<ProtocolRegistry>,
    pub registrar: Arc<RwLock<HashMap<String, Box<dyn Fn(Config) -> Box<dyn Device>>>>>,
}

impl DeviceFactory {
    /// Creates a new `DeviceFactory` with an associated protocol registry.
    pub fn new(protocol_registry: Arc<ProtocolRegistry>) -> Self {
        DeviceFactory {
            protocol_registry,
            registrar: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers and creates a device using a matching protocol handler.
    pub fn register(&self, config: &Config) -> Result<Box<dyn Device>, String> {
        for protocol in &config.supported_protocols {
            // Fetch handlers for each protocol
            if let Some(handlers) = self.protocol_registry.get_handlers(protocol) {
                // Attempt to find preferred handler, if specified
                if let Some(preferred) = &config.preferred_handler {
                    if let Some(handler) = handlers
                        .iter()
                        .find(|h| h.read().unwrap().name() == *preferred)
                    {
                        return handler.write().unwrap().create_device(config);
                    } else {
                        return Err(format!(
                            "Preferred handler '{}' not found for protocol: {}",
                            preferred, protocol
                        ));
                    }
                }

                // Use the highest priority handler if no preferred handler is specified
                if let Some(handler) = handlers.get(0) {
                    return handler.write().unwrap().create_device(config);
                }
            }
        }

        // Return an error if no compatible handler was found
        Err(format!(
            "No compatible protocol handler found for device: {}",
            config.name
        ))
    }
}
