# Blinkie: Home Automation System

### 1. Overview
Blinkie is a home automation system written in Rust, designed to be an alternative to Home Assistant. It manages IoT devices and allows for automation while prioritizing user-friendly configuration management and extensibility for developers and power-users.

The key design goal is to provide an intuitive, straightforward experience for users and reduce the complexities associated with home automation while maintaining robust functionality and extensibility.

### 2. Objectives
1. **Modular Device & Integration Management:**  
   Blinkie should support IoT devices in a protocol-agnostic manner through pluggable integrations (e.g., MQTT, Zigbee, HTTP APIs).
   
2. **Automation:**  
   Automation rules for controlling devices and workflows based on states or triggers.

3. **Extensibility:**  
   A plugin architecture allowing developers and power-users to add their integrations, as well as integrate other platforms (e.g., messenger bots, monitoring tools, Minecraft server plugins).

4. **Web-based Interface and API:**
   - Responsive web UI
   - REST API for external access and configuration

5. **Improved Configuration Management:**  
   Make it impossible for users (or the web UI) to misconfigure the system.

### 3. Architecture

#### 3.1 Core Components:
1. **Hub:**  
   Manages communication with devices, executes automation rules, and tracks device states.
   
2. **Integrations:**  
   Pluggable modules that manage communication with specific IoT protocols or other services (e.g., MQTT, Zigbee, HTTP APIs). Integrations may be for IoT devices or other platforms that Blinkie interacts with.

3. **REST API / Web Interface:**  
   The web interface relies on the REST API and provides access to monitor device states, manage configurations, integrations, and automation rules.

#### 3.1.1 Hub:
Handles:
- **Device Communication:**  
  Communicates with IoT devices and external platforms, ensuring proper event propagation and state consistency.
  
- **Automation Engine:**  
  Manages automation rules triggered by device state or external events.

- **State Manager:**  
  Tracks the state of devices and ensures the persistence of critical information. State updates trigger appropriate automation rules or inform other services (via integrations or webhooks).

- **Event System:**  
  Sends notifications or updates based on trigger actions (e.g., emitting an alert if a door sensor is triggered).

#### 3.1.2 Automation Engine:
Handles automation with:
- **Triggers:** Device changes, time-based events, or custom triggers.
- **Actions:** Executed when triggers are met.
- **Conditions:** Must be true for actions to execute.

#### 3.1.3 Integrations:
Each integration corresponds to a protocol or service that the system interacts with. Key responsibilities include:
- **Discovery:** Identifying supported devices.
- **Setup and Configuration:** Assisting with device configuration.
- **Device State Management:** Managing communication and state updates.
- **API Communication:** Facilitating communication between devices and services.

Blinkie aims to create its own integration architecture but will provide compatibility with the Home Assistant integration format where possible.


Supports:
- **If-This-Then-That (IFTTT)** style rules.
- **State-based automation:** Executes automations based on device states.

### 4. Development Plan

#### 4.1 MVP (Minimum Viable Product):
1. **Core Hub:**  
   Initial setup with a basic automation engine and device management system.
   
2. **MQTT Integration:**  
   Proof of concept integration for IoT devices using the MQTT protocol.
   
3. **REST API:**  
   An API providing access to the system.

4. **Basic Web UI:**  
   A simple web interface for monitoring and configuration.

5. **Dynamic Configuration Loading:**  
   Real-time configuration updates with proper validation.

#### 5.2 Phase 2
1. **Additional Integrations:**  
   Expand the integration/plugin ecosystem and aim to support Home Assistant modules.

#### 5.3 Long-term Goals
1. **Extensible Integration API:**  
   Excellent support for user-contributed integrations, with clear tooling and documentation for developers and power-users.



### Feature wish list
- Roles and permission management
