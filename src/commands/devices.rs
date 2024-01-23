use anyhow::{Context, Result};
use serialport::SerialPortType;

use crate::args::DevicesArgs;

pub fn run(_args: &DevicesArgs) -> Result<()> {
    let ports = serialport::available_ports().context("Error getting serial ports.")?;

    if ports.is_empty() {
        println!("[*] No serial ports found.");
        return Ok(());
    }

    println!("[*] Available Ports ({})", ports.len());
    for (i, port) in ports.iter().enumerate() {
        let connection = match port.port_type {
            SerialPortType::UsbPort(_) => "USB",
            SerialPortType::PciPort => "PCI",
            SerialPortType::BluetoothPort => "Bluetooth",
            SerialPortType::Unknown => "Unknown",
        };
        println!(
            " {} {} ({connection})",
            if i + 1 == ports.len() { "└" } else { "├" },
            port.port_name
        );

        if let SerialPortType::UsbPort(usb) = &port.port_type {
            if let Some(product) = usb.product.as_ref() {
                println!("   ├─ Product: {}", product);
            }
            if let Some(manufacturer) = usb.manufacturer.as_ref() {
                println!("   ├─ Manufacturer: {}", manufacturer);
            }
            if let Some(serial_number) = usb.serial_number.as_ref() {
                println!("   ├─ Serial Number: {}", serial_number);
            }
            println!("   ├─ Vendor ID: 0x{:04x}", usb.vid);
            println!("   └─ Product ID: 0x{:04x}", usb.pid);
        }
    }

    Ok(())
}
