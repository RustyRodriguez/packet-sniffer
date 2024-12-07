mod device;
mod capture;

use device::get_device;
use capture::sniff_packets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = get_device("wlp2s0")?;
    println!("Using device {}", device.name);

    sniff_packets(device)?;

    Ok(())
}