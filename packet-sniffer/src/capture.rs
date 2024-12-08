use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use pcap::Device;
use crate::protocol::identify_protocol;

pub fn sniff_raw_packets(network_interface: Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut packet_capture = network_interface.open()?;

    while let Ok(packet) = packet_capture.next_packet() {
        println!("Packet Captured: {:?}", packet);
    }

    Ok(())
}


pub fn sniff_protocol_packets(network_interface: Device) -> Result<(), Box<dyn std::error::Error>> {
    let mut packet_capture = network_interface.open()?;

    let mut protocols = Arc::new(Mutex::new(HashSet::new()));

    let reporting_thread_protocols = Arc::clone(&protocols);

    thread::spawn(
        move || {
            loop {
                thread::sleep(Duration::from_secs(10));

                let protocols = reporting_thread_protocols.lock().unwrap();

                println!("Unique protocols = {:?}", *protocols);
            }
        }
    );

    while let Ok(packet) = packet_capture.next_packet() {
        if let Some(protocol) = identify_protocol(packet.data) {

            let mut protocols = protocols.lock().unwrap();
            protocols.insert(protocol);
        }
    }

    Ok(())
}