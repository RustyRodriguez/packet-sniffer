use pcap::Device;

pub fn get_device(device_name: &str) -> Result<Device, Box<dyn std::error::Error>> {
    let devices = Device::list()?;

    println!("Available devices on this system: ");

    for device in &devices {
        println!(" {}", device.name);
    }

    devices.into_iter().find(|d| d.name == device_name)
        .ok_or_else(|| format!("Device {} not found", device_name).into())

}