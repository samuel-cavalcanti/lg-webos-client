use regex::Regex;
use tokio::{io, net::UdpSocket};

pub struct MagicPacket {
    magic_bytes: Vec<u8>,
}

/// let mac_bytes = string_to_bytes_mac_address("a4:a2:22:a7:ff:3f").unwrap();
/// assert_eq!(vec![0xa4,0xa2,0x22,0xa7,0xff,0x3f], mac_bytes);
pub fn string_to_bytes_mac_address(mac_address: &str) -> Option<Vec<u8>> {
    let hex = r"([[:xdigit:]]{2})";
    let regex = Regex::new(&format!("{hex}:{hex}:{hex}:{hex}:{hex}:{hex}")).unwrap();

    let caps = regex.captures(mac_address)?;

    let mut mac_bytes = Vec::with_capacity(6);
    for i in 1..7 {
        let match_hex = caps.get(i)?;
        let hex_digit = u8::from_str_radix(match_hex.as_str(), 16).unwrap();
        mac_bytes.push(hex_digit);
    }

    Some(mac_bytes)
}

impl MagicPacket {
    pub fn from_mac_string(mac_address: &str) -> Option<MagicPacket> {
        let bytes = string_to_bytes_mac_address(mac_address)?;

        let packet = MagicPacket::new(bytes);

        Some(packet)
    }
    pub fn new(mac_address: Vec<u8>) -> MagicPacket {
        /*
        The magic packet is a frame that is most often sent as a broadcast
        and that contains anywhere within its payload 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal),
        followed by sixteen repetitions of the target computer's 48-bit MAC address, for a total of 102 bytes.
        */
        let repeted_mac = mac_address.repeat(16);
        let header = [255].repeat(6);

        let mut magic_bytes = header;
        magic_bytes.extend(repeted_mac);

        MagicPacket { magic_bytes }
    }
}
/// Sends the magic packet to default broadcast address
/// Maybe the TV will not receive the  packet
pub async fn send_magic_packet_to_broadcast(package: MagicPacket) -> io::Result<()> {
    let broadcast_address = "255:255:255:255:9";
    let socket = UdpSocket::bind("0.0.0.0:8000").await?;

    socket.set_broadcast(true)?;

    socket
        .send_to(package.magic_bytes.as_slice(), broadcast_address)
        .await?;
    Ok(())
}

/// Sends the magic packet to especific address
pub async fn send_magic_packet_to_address(
    package: MagicPacket,
    ip_address: &str,
) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8000").await?;
    socket
        .send_to(package.magic_bytes.as_slice(), ip_address)
        .await?;

    Ok(())
}

#[test]
fn test_magic_packet_from_strimg() {
    let macs = ["a4:a2:22:a7:ff:3f", "f4:b2:a2:f7:af:4f"];
    let macs_bytes: [Vec<u8>; 2] = [
        vec![0xa4, 0xa2, 0x22, 0xa7, 0xff, 0x3f],
        vec![0xf4, 0xb2, 0xa2, 0xf7, 0xaf, 0x4f],
    ];

    let assert_bytes = |bytes: Vec<u8>, expected_bytes: Vec<u8>| {
        for (byte, expected_byte) in bytes.iter().zip(expected_bytes) {
            assert_eq!(*byte, expected_byte);
        }
    };
    for (mac_string, mac_bytes) in macs.iter().zip(macs_bytes) {
        let packet = MagicPacket::new(mac_bytes.clone());
        let packet_from_string = MagicPacket::from_mac_string(mac_string).unwrap();
        let bytes = string_to_bytes_mac_address(mac_string).unwrap();

        assert_bytes(bytes, mac_bytes);
        assert_bytes(packet.magic_bytes, packet_from_string.magic_bytes);
    }

    let wrong_format = "a4;a2;22;a7;ff;3f";
    let bytes = string_to_bytes_mac_address(wrong_format);

    assert_eq!(bytes, None);
}

#[test]
fn test_magic_packet_from_mac_bytes() {
    let mac_address = vec![1, 2, 3, 4, 5, 6];
    let packet = MagicPacket::new(mac_address);

    assert_eq!(packet.magic_bytes.len(), 102);
    assert_eq!(packet.magic_bytes[0..6], [255, 255, 255, 255, 255, 255]);
    assert_eq!(packet.magic_bytes[6..6 + 6], [1, 2, 3, 4, 5, 6]);
    assert_eq!(packet.magic_bytes[102 - 6..102], [1, 2, 3, 4, 5, 6]);
}
