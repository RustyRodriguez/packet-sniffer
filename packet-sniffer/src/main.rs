mod device;
mod capture;
mod protocol;

use device::get_device;
use capture::{sniff_protocol_packets, sniff_raw_packets};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = get_device("wlp2s0")?;
    println!("Using device {}", device.name);

    sniff_protocol_packets(device)?;

    Ok(())
}