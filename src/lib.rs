
mod app;
mod remote;
mod device;
mod client;
mod ssdp;

// Re-export higher-level stuff
pub use crate::app::*;
pub use crate::remote::*;
pub use crate::device::*;
pub use crate::ssdp::discover_devices;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn try_discover_devices() {
        println!("[-] Attempting device discovery...");
        match discover_devices(Duration::new(5, 0)).await {
            Ok(devices) => {
                // Print the names of the discovered devices
                println!("[+] Discovered devices:\n------------------------------");
                devices.iter().for_each(|d| println!("Name: {}\nIP: {}\nPort: {}\nMAC: {:02x?}", d.name, d.ipv4, d.port, d.mac_wlan));
                println!("------------------------------");
                assert_ne!(devices.len(), 0)
            }
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    async fn power_on_device() {
        // This tries to power on the first discovered device
        match discover_devices(Duration::new(5, 0)).await {
            Ok(devices) => {
                match devices[0].send_power_command(POWERCOMMAND::TURNON).await {
                    Ok(response) => assert!(response),
                    _ => assert!(false)
                }
            }
            Err(_) => assert!(false)
        }

    }
}

