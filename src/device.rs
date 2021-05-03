use std::collections::HashMap;
use std::time::Duration;
use quick_xml::{Reader, events::Event};
use crate::{client, App};
use std::ops::Deref;
use wake_on_lan::MagicPacket;
use reqwest::StatusCode;
use std::str::FromStr;

/// Device object
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub ipv4:       String,
    pub port:       i32,
    pub name:       String,
    pub network:    NETWORKTYPE,
    pub mac_wlan:   [u8; 6],
    pub mac_eth:    [u8; 6],
}

impl Device {

    /// Return parsed device-info XML
    pub async fn get_info(&self) -> Result<HashMap<String, String>, String> {
        // GET device-info endpoint
        match client::get(&self.ipv4,"query/device-info", Duration::new(3, 0)).await {
            // Parse response
            Ok(xml) => {
                // Parsed XML keys/values
                let mut xml_parsed: HashMap<String, String> = HashMap::new();
                // Create XML reader
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                // XML event buffer
                let mut buffer = Vec::new();
                // Current tag
                let mut tag = String::new();
                // Loop the XML
                loop {
                    match reader.read_event(&mut buffer) {
                        // Read each tag
                        Ok(Event::Start(ref e)) => tag = std::str::from_utf8(e.name()).unwrap_or("").to_string(),
                        // Handle tag content
                        Ok(Event::Text(e)) => {
                            // Skip working with top-level tags
                            if tag != "?xml" && tag != "device-info" {
                                // Create new entry in hashmap
                                xml_parsed.insert(
                                    tag.clone(),
                                    e.unescape_and_decode(&reader).unwrap_or(String::new())
                                );
                            }
                        },
                        // Break at EOF
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buffer.clear();
                }
                // Return hashmap of xml
                Ok(xml_parsed)
            },
            Err(e) => Err(e.to_string())
        }
    }

    /// Get device power state
    pub async fn get_power_state(&self) -> POWERSTATE {
        let mut power_state = POWERSTATE::UNKNOWN;
        match client::get(&self.ipv4, "query/device-info", Duration::new(3, 0)).await {
            // Parse the response we received
            Ok(response) => {
                // Create XML reader
                let mut reader = Reader::from_str(&response);
                reader.trim_text(true);
                // XML event buffer
                let mut buffer = Vec::new();
                // Whether or not to read the tag
                let mut read = false;
                // Loop the XML
                loop {
                    match reader.read_event(&mut buffer) {
                        // Read each tag
                        Ok(Event::Start(ref e)) => {
                            if e.name() == b"power-mode" {
                                read = true;
                            }
                        },
                        // Return parsed content of the <power-mode> tag
                        Ok(Event::Text(e)) => {
                            if read {
                                power_state = POWERSTATE::from(e.unescape_and_decode(&reader).unwrap().to_ascii_uppercase())
                            }
                            // Stop reading tags
                            read = false;
                        },
                        // Break at EOF
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buffer.clear();
                }
            }
            // If request timed out, assume 'Off'
            Err(e) => {
                if e == StatusCode::REQUEST_TIMEOUT {
                    power_state = POWERSTATE::OFF;
                }
            }
        }
        power_state
    }

    /// Change device power state and return whether or not it worked.
    /// NOTE: This will attempt to wake a device that is turned off.
    pub async fn send_power_command(&self, command: POWERCOMMAND) -> Result<bool, String> {
        // Get current device state
        let current_state = self.get_power_state().await;
        // Result of attempt to change state
        let mut result: Result<bool, String> = Ok(true);
        // Handle the provided command
        match command {
            POWERCOMMAND::TURNOFF => {
                // Turn off if on
                if current_state == POWERSTATE::ON {
                    // Send PowerOff key to device
                    result = match client::post(&self.ipv4, "keypress/PowerOff", None, Duration::new(5, 0)).await {
                        Ok(_) => Ok(true),
                        Err(e) => Err(e.to_string())
                    };
                }
            }
            POWERCOMMAND::TOGGLE => {
                match current_state {
                    // Turn off if on
                    POWERSTATE::ON => {
                        // Send PowerOff key to device
                        result = match client::post(&self.ipv4, "keypress/PowerOff", None, Duration::new(5, 0)).await {
                            Ok(_) => Ok(true),
                            Err(e) => Err(e.to_string())
                        };
                    },
                    // Turn on if off
                    POWERSTATE::DISPLAYOFF => {
                        // Send undocumented PowerOn key to device
                        result = match client::post(&self.ipv4, "keypress/PowerOn", None, Duration::new(5, 0)).await {
                            Ok(_) => Ok(true),
                            Err(e) => Err(e.to_string())
                        };
                    }
                    // Send W-o-L if powered down or unknown
                    _ => {
                        result = match MagicPacket::new(if self.network == NETWORKTYPE::ETHERNET { &self.mac_eth } else { &self.mac_wlan }).send() {
                            Ok(..) => Ok(true),
                            Err(_) => Err(String::from("Unable to send Wake-on-LAN"))
                        }
                    }
                }
            }
            // Assume command "ON" by default
            _ => {
                match current_state {
                    // Turn on if off
                    POWERSTATE::DISPLAYOFF => {
                        // Send undocumented PowerOn key to device
                        result = match client::post(&self.ipv4, "keypress/PowerOn", None, Duration::new(5, 0)).await {
                            Ok(_) => Ok(true),
                            Err(e) => Err(e.to_string())
                        };
                    },
                    // Send W-o-L if powered down or unknown
                    POWERSTATE::OFF | POWERSTATE::UNKNOWN => {
                        result = match MagicPacket::new(if self.network == NETWORKTYPE::ETHERNET { &self.mac_eth } else { &self.mac_wlan }).send() {
                            Ok(..) => Ok(true),
                            Err(_) => Err(String::from("Unable to send Wake-on-LAN"))
                        }
                    }
                    // Do nothing if already on
                    POWERSTATE::ON => ()
                }
            }
        }

        result
    }

    /// Get list of installed apps
    pub async fn get_installed_apps(&self) -> Result<Vec<App>, String> {
        // GET device-info endpoint
        let xml_raw = client::get(&self.ipv4,"query/apps", Duration::new(3, 0)).await;
        match xml_raw {
            Ok(xml) => {
                // Parsed XML keys/values
                let mut apps_parsed: Vec<App> = Vec::new();
                // Create XML reader
                let mut reader = Reader::from_str(&xml);
                reader.trim_text(true);
                // XML event buffer
                let mut buffer = Vec::new();
                // Whether to read tag content
                let mut read = false;
                // Current roku app from tag
                let mut app = App {
                    id: 0,
                    apptype: "".to_string(),
                    version: "".to_string(),
                    name: "".to_string(),
                    icon: None
                };
                // Loop the XML
                loop {
                    match reader.read_event(&mut buffer) {
                        // Read each tag
                        Ok(Event::Start(ref e)) => {
                            if e.name() != b"?xml" && e.name() != b"apps" {
                                // Parse and collect attributes
                                let attributes = e.attributes()
                                    .map(|a| a.unwrap().value)
                                    .collect::<Vec<_>>();
                                // Create RokuApp object from attributes
                                app = App {
                                    id: i32::from_str(&std::str::from_utf8(attributes[1].deref()).unwrap_or("").to_string()).unwrap(),
                                    apptype: std::str::from_utf8(attributes[1].deref()).unwrap_or("").to_string(),
                                    version: std::str::from_utf8(attributes[2].deref()).unwrap_or("").to_string(),
                                    name: String::new(),
                                    icon: None
                                };
                                // Prepare to read tag content
                                read = true;
                            }
                        },
                        // Handle tag content
                        Ok(Event::Text(e)) => {
                            // Skip working with top-level tags
                            if read {
                                // Update currently-parsed app name
                                app.name = e.unescape_and_decode(&reader)
                                    .unwrap_or(String::new())
                                    .replace("\u{a0}", "");     // There are newline characters in some names
                                // Add app to list of parsed apps
                                apps_parsed.push(app.clone());
                            }
                        },
                        // Break at EOF
                        Ok(Event::Eof) => break,
                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                        _ => (),
                    }
                    buffer.clear();
                }
                // Return list of apps
                Ok(apps_parsed)
            },
            Err(e) => Err(e.to_string())
        }
    }

    /// Launch an app by its id with a waking POST (useful for cold-launching)
    pub async fn launch_app_by_id(&self, app_id: i32) -> Result<bool, String> {
        match client::waking_post(
            &self.ipv4,
            // Choose MAC based on network type
            if self.network == NETWORKTYPE::ETHERNET {&self.mac_eth} else {&self.mac_wlan},
            &format!("launch/{}", app_id),
            Duration::new(3, 0)
        ).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string())
        }
    }

    /// Manually update this object to match real-world device
    pub async fn update_self(&mut self) {
        // Attempt to get complete device info (we currently only have IP & port)
        match self.get_info().await {
            Ok(info) => {
                // Update device object with new info using the hashmap
                self.name = info.get("friendly-device-name").unwrap().clone();
                self.network = NETWORKTYPE::from(info.get("network-type").unwrap().clone().to_ascii_uppercase());
                self.mac_wlan = split_mac(info.get("wifi-mac").unwrap());
                // Handle failing to resolve this from the hashmap (do devices w/o support still have it?)
                if let Some(support) = info.get("supports-ethernet") {
                    // Check if this device supports ethernet
                    if support.to_ascii_uppercase().as_str() == "TRUE" {
                        // Parse the Ethernet MAC
                        self.mac_eth = split_mac(info.get("ethernet-mac").unwrap_or(&"0:0:0:0:0:0".to_string()))
                    }
                }
            },
            Err(_) => {

            }
        }
    }

    /// Factory
    #[inline]
    pub fn new() -> Device {
        Device {
            ipv4: "".to_string(),
            port: 0,
            name: "".to_string(),
            network: NETWORKTYPE::WIRELESS,
            mac_wlan: [0; 6],
            mac_eth: [0; 6]
        }
    }
    /// Factory w/ only IPv4 and port
    #[inline]
    pub fn from_ipv4(ipv4: &str, port: i32) -> Device {
        Device {
            ipv4: String::from(ipv4),
            port,
            name: "".to_string(),
            network: NETWORKTYPE::WIRELESS,
            mac_wlan: [0; 6],
            mac_eth: [0; 6]
        }
    }
}

/// Network types a device could be connected to
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NETWORKTYPE {
    WIRELESS,   // e.g. Wi-Fi
    ETHERNET,   // Ethernet cable
}

impl ToString for NETWORKTYPE {
    fn to_string(&self) -> String {
        match self {
            NETWORKTYPE::WIRELESS => String::from("WIRELESS"),
            NETWORKTYPE::ETHERNET => String::from("ETHERNET")
        }
    }
}

// Assumes NETWORKTYPE::WIRELESS
impl From<String> for NETWORKTYPE {
    fn from(s: String) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "ETHERNET" => NETWORKTYPE::ETHERNET,
            _ => NETWORKTYPE::WIRELESS
        }
    }
}

/// Possible power states for a device to be in
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum POWERSTATE {
    OFF,        // Powered down, requires wake-on-lan
    DISPLAYOFF, // Screen off, hardware on, still accessible via API
    ON,         // Screen is on
    UNKNOWN,    // ???
}

impl From<String> for POWERSTATE {
    fn from(s: String) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "OFF" | "POWEROFF" => POWERSTATE::OFF,
            "DISPLAYOFF" => POWERSTATE::DISPLAYOFF,
            "ON" | "POWERON" => POWERSTATE::ON,
            _ => POWERSTATE::UNKNOWN
        }
    }
}

impl ToString for POWERSTATE {
    fn to_string(&self) -> String {
        match self {
            POWERSTATE::OFF => String::from("Off"),
            POWERSTATE::DISPLAYOFF => String::from("DisplayOff"),
            POWERSTATE::ON => String::from("On"),
            POWERSTATE::UNKNOWN => String::from("Unknown"),
        }
    }
}

/// Power state change commands to send to a device
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum POWERCOMMAND {
    TURNOFF,        // Power off the device
    TURNON,         // Power on the device
    TOGGLE,     // Set device to opposite state
}

// Assumes POWERCOMMAND::TURNON
impl From<String> for POWERCOMMAND {
    fn from(s: String) -> Self {
        // Assume "ON" if we're not sure
        match s.to_ascii_uppercase().as_str() {
            "OFF" => POWERCOMMAND::TURNOFF,
            "TOGGLE" => POWERCOMMAND::TOGGLE,
            _ => POWERCOMMAND::TURNON
        }
    }
}

/// Split up device MACs into byte arrays
fn split_mac(input: &str) -> [u8; 6] {
    let mut index = 0;
    let mut output: [u8; 6] = [0; 6];
    str::split(input, ':')
        .for_each(|chunk| {
            output[index] = u8::from_str_radix(chunk, 16).unwrap();
            index += 1;
        });
    output
}