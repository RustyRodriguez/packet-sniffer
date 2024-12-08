use std::collections::HashSet;

pub fn identify_protocol(packet: &[u8]) -> Option<String> {
    
    if packet.len() < 14 {
        return None;
    }

    let ethertype = u16::from_be_bytes([packet[12], packet[13]]);

    match ethertype {
        0x0800 => {
            if packet.len() < 34 {
                return None; 
            }
            let protocol = packet[23];
            match protocol {
                1 => Some("ICMP".to_string()),
                6 => Some("TCP".to_string()),
                17 => Some("UDP".to_string()),
                _ => Some(format!("Unknown IPv4 protocol: {}", protocol)),
            }
        }
        0x86DD => Some("IPv6".to_string()), 
        0x0806 => Some("ARP".to_string()),
        _ => Some(format!("Unknown EtherType: 0x{:04x}", ethertype)),
    }
}