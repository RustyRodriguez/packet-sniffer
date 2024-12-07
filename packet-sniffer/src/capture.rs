use pcap::Device;

pub fn sniff_packets(network_interface: Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut packet_capture = network_interface.open()?;

    while let Ok(packet) = packet_capture.next_packet() {
        println!("Packet Captured: {:?}", packet);
    }

    Ok(())
}