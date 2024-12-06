use pcap::Device;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::lookup()?.ok_or("No device found")?;
    println!("Using device: {}", device.name);

    let mut cap = device.open()?;
    println!("Device opened for capturing!");

    while let Ok(packet) = cap.next_packet() {
        println!("Received packet: {:?}", packet);
    }

    Ok(())
}