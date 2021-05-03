/// Emulate use of a remote control, and help locate one
use crate::{Device, client};
use std::time::Duration;

/// Adds additional remote-control
// NOTE: These require the device to be powered on
impl Device {
    /// Press a button on the remote
    // IMPLEMENTATION NOTE: If implementing a remote UI, it's best to use Device.set_power_state(TOGGLE) instead of sending PowerOn/PowerOff button presses
    pub async fn press_button(&self, button: BUTTON) -> Result<bool, String> {
        match client::post(&self.ipv4, &format!("keypress/{}", button.to_string()), None, Duration::new(5, 0)).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string())
        }
    }

    /// Send multiple button presses back-to-back
    pub async fn press_buttons(&self, buttons: Vec<BUTTON>) -> Result<bool, String> {
        let mut result = Ok(true);
        // Send buttons until we reach the end, stop if one doesn't send
        for b in buttons.into_iter() {
            if let Err(e) = self.press_button(b).await {
                result = Err(e);
                break;
            }
        }
        result
    }

    /// Send UTF-8 character literal as though typed on the remote
    pub async fn press_key(&self, key: char) -> Result<bool, String> {
        let keycode = format!("Lit_{}", urlencoding::encode(&key.to_string()));
        match client::post(&self.ipv4, &format!("keypress/{}", keycode), None, Duration::new(5, 0)).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string())
        }
    }

    /// Send UTF-8 string as series of characters as though typed on the remote
    pub async fn press_keys(&self, input: &str) -> Result<bool, String> {
        let mut result = Ok(true);
        // Send keys until we reach the end, stop if one doesn't send
        for c in input.chars().into_iter() {
            if let Err(e) = self.press_key(c).await {
                result = Err(e);
                break;
            }
        }
        result
    }

    // TODO: Implement "Find Remote" capability
    // NOTE: My device is too cheap/old to have this, so not sure how I'm going to test it...
    pub async fn find_remote(&self) {
        todo!()
    }
}

/// All known remote buttons
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BUTTON {
    Back,
    Backspace,
    ChannelUp,      // Requires device support
    ChannelDown,    // Requires device support
    Down,
    Enter,
    FindRemote,     // Requires device support
    Fwd,
    Home,
    Info,
    InputTuner,     // Requires device support
    InputHDMI1,     // Requires device support
    InputHDMI2,     // Requires device support
    InputHDMI3,     // Requires device support
    InputHDMI4,     // Requires device support
    InputAV1,       // Requires device support
    InstantReplay,
    Left,
    Play,
    Rev,
    Right,
    Search,
    Select,
    Up,
    VolumeDown,     // Requires device support
    VolumeMute,     // Requires device support
    VolumeUp,       // Requires device support
    PowerOff,       // Requires device support
    PowerOn,        // Undocumented by works on my device (^_^')
}
impl ToString for BUTTON {
    fn to_string(&self) -> String {
        match self {
            BUTTON::Back => String::from("Back"),
            BUTTON::Backspace => String::from("Backspace"),
            BUTTON::ChannelUp => String::from("ChannelUp"),
            BUTTON::ChannelDown => String::from("ChannelDown"),
            BUTTON::Down => String::from("Down"),
            BUTTON::Enter => String::from("Enter"),
            BUTTON::FindRemote => String::from("FindRemote"),
            BUTTON::Fwd => String::from("Fwd"),
            BUTTON::Home => String::from("Home"),
            BUTTON::Info => String::from("Info"),
            BUTTON::InputTuner => String::from("InputTuner"),
            BUTTON::InputHDMI1 => String::from("InputHDMI1"),
            BUTTON::InputHDMI2 => String::from("InputHDMI2"),
            BUTTON::InputHDMI3 => String::from("InputHDMI3"),
            BUTTON::InputHDMI4 => String::from("InputHDMI4"),
            BUTTON::InputAV1 => String::from("InputAV1"),
            BUTTON::InstantReplay => String::from("InstantReplay"),
            BUTTON::Left => String::from("Left"),
            BUTTON::Play => String::from("Play"),
            BUTTON::Rev => String::from("Rev"),
            BUTTON::Right => String::from("Right"),
            BUTTON::Search => String::from("Search"),
            BUTTON::Select => String::from("Select"),
            BUTTON::Up => String::from("Up"),
            BUTTON::VolumeDown => String::from("VolumeDown"),
            BUTTON::VolumeMute => String::from("VolumeMute"),
            BUTTON::VolumeUp => String::from("VolumeUp"),
            BUTTON::PowerOff => String::from("PowerOff"),
            BUTTON::PowerOn => String::from("PowerOn"),
        }
    }
}
impl From<String> for BUTTON {
    fn from(s: String) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "BACK" => BUTTON::Back,
            "BACKSPACE" => BUTTON::Backspace,
            "CHANNELUP" => BUTTON::ChannelUp,
            "CHANNELDOWN" => BUTTON::ChannelDown,
            "DOWN" => BUTTON::Down,
            "ENTER" => BUTTON::Enter,
            "FINDREMOTE" => BUTTON::FindRemote,
            "FWD" => BUTTON::Fwd,
            "HOME" => BUTTON::Home,
            "INFO" => BUTTON::Info,
            "INPUTTUNER" => BUTTON::InputTuner,
            "INPUTHDMI1" => BUTTON::InputHDMI1,
            "INPUTHDMI2" => BUTTON::InputHDMI2,
            "INPUTHDMI3" => BUTTON::InputHDMI3,
            "INPUTHDMI4" => BUTTON::InputHDMI4,
            "INPUTAV1" => BUTTON::InputAV1,
            "INSTANTREPLAY" => BUTTON::InstantReplay,
            "LEFT" => BUTTON::Left,
            "PLAY" => BUTTON::Play,
            "REV" => BUTTON::Rev,
            "RIGHT" => BUTTON::Right,
            "SEARCH" => BUTTON::Search,
            "SELECT" => BUTTON::Select,
            "UP" => BUTTON::Up,
            "VOLUMEDOWN" => BUTTON::VolumeDown,
            "VOLUMEMUTE" => BUTTON::VolumeMute,
            "VOLUMEUP" => BUTTON::VolumeUp,
            "POWEROFF" => BUTTON::PowerOff,
            _ => BUTTON::PowerOn
        }
    }
}