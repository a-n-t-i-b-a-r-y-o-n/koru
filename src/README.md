koru
====
_Roku client library written in Rust_

## Objects

### Device

#### Properties
* `ipv4:      String`
* `port:      i32`
* `usn:       Option<String>`
* `name:      String`
* `network:   NETWORKTYPE`
* `mac_wlan:  [u8; 6]`
* `mac_eth:   [u8; 6]`

#### Methods
* `get_info() : Result<HashMap<String, String>, String>`  
  Return parsed device info
* `get_power_state() : POWERSTATE`  
  Get device power state
* `send_power_command(command: POWERCOMMAND) : Result<bool, String>`  
  Git a power command, e.g. turn on, turn off, toggle
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
* `id: i32`
* `apptype: String`
* `version: String,`
* `name: String,`
* `icon: Option<Vec<u8>>`

#### Methods
* `fetch_icon()  :  Result<Vec<u8>, String>`  
  Fetches the icon from the device for this app