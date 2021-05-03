koru
====
_Roku client library in Rust_

This project provides objects to represent and interact with real-world Roku hardware, as well as an asynchronous Roku API client.
Brief overview of features:
* Discover Roku devices on the local network
* Interact with Roku devices, e.g.
  * Get device info & installed apps
  * Wake them from sleep or turn them off
  * Press buttons on the remote
  * Send key presses
  * Launch apps

## Discovery
`discover_devices() -> Result<Vec<Device>, Error>`  
Attempt to discover devices with SSDP, then return the list of responders.

## Objects

### Device

#### Properties
* `ipv4:      String` - IPv4 Address
* `port:      i32` - Port number
* `name:      String` - Friendly device name
* `network:   NETWORKTYPE` - Network type, e.g. Ethernet or Wireless
* `mac_wlan:  [u8; 6]` - MAC Address for WLAN
* `mac_eth:   [u8; 6]` - MAC Address for Ethernet

#### Methods
* `get_info() : Result<HashMap<String, String>, String>`  
  Return parsed device info
* `get_power_state() : POWERSTATE`  
  Get device power state
* `send_power_command(command: POWERCOMMAND) : Result<bool, String>`  
  Send a power command, e.g. turn on, turn off, toggle
* `get_installed_apps() : Result<Vec<App>, String>`  
  Return a Vec of installed apps
* `launch_app_by_id(app: &App) : Result<bool, String>`  
  Launches an app of specified id with a wakeful POST
* `press_button(button: BUTTON) -> Result<bool, String>`  
  Emulates pressing a button on the remote
* `press_buttons(buttons: Vec<BUTTON>) -> Result<bool, String>`  
  Emulates pressing multiple buttons on the remote back-to-back
* `press_key(key: char) -> Result<bool, String>`  
  Emulates entering a keystroke
* `press_keys(input: &str) -> Result<bool, String>`  
  Emulates entering multiple keystrokes to type a string

### App

#### Properties
* `id: i32` - App ID
* `apptype: String` - App type, e.g. tvtuner, app
* `version: String,` - App version
* `name: String,` - Friendly app name
* `icon: Option<Vec<u8>>` - App Icon, potentially unfetched

#### Methods
* `fetch_icon()  :  Result<Vec<u8>, String>`  
  Fetches the icon from the device for this app