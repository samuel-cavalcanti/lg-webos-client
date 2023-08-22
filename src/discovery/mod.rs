use std::{collections::HashMap, net::SocketAddr};

use regex::Regex;
use ssdp::{
    header::{HeaderMut, HeaderRef, Man, Server, MX, ST},
    message::{Multicast, SearchRequest, SearchResponse},
    SSDPError,
};

#[derive(Debug, Clone)]
pub struct WebOsNetworkInfo {
    pub name: String,
    pub ip: String,
    // The mac address is need to Turn on the TV using Wake On LAN (WOL)
    pub mac_address: String,
}

/*
    SSDP mean Simple Service Discovery Protocol
    this will send SSDP search packages find the TV. The TV must be on.
*/
pub async fn discovery_webostv_by_ssdp() -> Result<Vec<WebOsNetworkInfo>, SSDPError> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(ST::All);
    request.set(MX(5));
    let responses = match request.multicast() {
        Ok(responses) => responses,
        Err(e) => {
            log::error!("Error on send multicast request. Error:{e:?}");
            return Err(e);
        }
    };

    let mut tvs: HashMap<String, _> = HashMap::new();
    for (response, src) in responses {
        log::trace!("ssdp reponse: {response:?}");
        let tv_network = parse_response(response, src);
        log::trace!("Found TV: {tv_network:?}");
        if let Some(tv) = tv_network {
            tvs.insert(tv.ip.clone(), tv);
            //tvs.push(tv);
        }
    }
    Ok(tvs.into_values().collect())
}

fn parse_response(response: SearchResponse, address: SocketAddr) -> Option<WebOsNetworkInfo> {
    let st = response.get::<ST>()?;
    let server = response.get::<Server>()?;
    let wake_up = response.get_raw("WAKEUP")?;

    is_webos_package(st)?;

    let mac_address = wake_up_bytes_to_string(&wake_up[0])?;

    let info = WebOsNetworkInfo {
        name: server.to_string(),
        ip: address.ip().to_string(),
        mac_address,
    };

    Some(info)
}
fn is_webos_package(st: &ST) -> Option<()> {
    let webos_filter: ST = ST::Target(ssdp::FieldMap::URN(
        "dial-multiscreen-org:service:dial:1".to_string(),
    ));
    if *st != webos_filter {
        return None;
    }
    Some(())
}

fn wake_up_bytes_to_string(bytes: &[u8]) -> Option<String> {
    let mac_regex: Regex = Regex::new(r"MAC=(?P<mac_address>.?+);").unwrap();
    let wake_up = std::str::from_utf8(bytes).unwrap_or("");
    let address = mac_regex
        .captures(wake_up)?
        .name("mac_address")?
        .as_str()
        .to_string();
    Some(address)
}

#[test]
fn test_wake_up_bytes_to_string() {
    let array_bytes = [
        b"MAC=04:a2:22:a6:0f:3e;Timeout=60".to_vec(),
        b"MAC=12:12:22:af:0f:3a;Timeout=60".to_vec(),
        b"MAC=13:42:52:aa:af:3f;".to_vec(),
        vec![],
    ];
    let expecteds = [
        Some("04:a2:22:a6:0f:3e".to_string()),
        Some("12:12:22:af:0f:3a".to_string()),
        Some("13:42:52:aa:af:3f".to_string()),
        None,
    ];

    for (bytes, expected_address) in array_bytes.iter().zip(expecteds) {
        let option = wake_up_bytes_to_string(bytes);
        assert_eq!(option, expected_address);
    }
}

#[test]
fn test_is_webos_packege() {
    let sts = [
        ST::Target(ssdp::FieldMap::URN(
            "dial-multiscreen-org:service:dial:1".to_string(),
        )),
        ST::Target(ssdp::FieldMap::UPnP("rootdevice".to_string())),
        ST::Target(ssdp::FieldMap::UUID(
            "f2b52f59-e365-3978-4ddf-6a85551bd9ee".to_string(),
        )),
    ];

    let expecteds = [Some(()), None, None];

    for (st, expected) in sts.iter().zip(expecteds) {
        let option = is_webos_package(st);

        assert_eq!(option, expected);
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr};

    use super::*;
    use ssdp::message::SearchResponse;

    #[test]
    fn test_good_response() {
        /*
           Cache-Control: max-age=1800
           Server: WebOS/1.5 UPnP/1.0 webOSTV/1.0
           EXT:
           USN: uuid:a09767b7-fc36-46b6-96db-061011ca9647::urn:dial-multiscreen-org:service:dial:1
           ST: urn:dial-multiscreen-org:service:dial:1
           Date: Fri, 24 Feb 2023 00:54:40 GMT
           WAKEUP: MAC=04:a2:22:a6:0f:3e;Timeout=60
        */
        let mut good_response = SearchResponse::new();
        good_response.set_raw("Cache-Control", vec![b"max-age=1800".to_vec()]);
        good_response.set_raw("Server", vec![b"WebOS/1.5 UPnP/1.0 webOSTV/1.0".to_vec()]);
        good_response.set_raw("EXT", vec![vec![]]);
        good_response.set_raw(
            "USN",
            vec![
            b"uuid:a09767b7-fc36-46b6-96db-061011ca9647::urn:dial-multiscreen-org:service:dial:1"
                .to_vec(),
        ],
        );
        good_response.set_raw(
            "ST",
            vec![b"urn:dial-multiscreen-org:service:dial:1".to_vec()],
        );

        good_response.set_raw("Date", vec![b"Fri, 24 Feb 2023 00:54:40 GMT".to_vec()]);

        good_response.set_raw("WAKEUP", vec![b"MAC=04:a2:22:a6:0f:3e;Timeout=60".to_vec()]);

        let ip = Ipv4Addr::new(192, 168, 0, 199);
        let address = SocketAddr::new(std::net::IpAddr::V4(ip), 1084);
        let info = parse_response(good_response, address).unwrap();

        let expeced_info = WebOsNetworkInfo {
            name: "WebOS/1.5 UPnP/1.0 webOSTV/1.0".to_string(),
            ip: "192.168.0.199".to_string(),
            mac_address: "04:a2:22:a6:0f:3e".to_string(),
        };

        assert_eq!(info.name, expeced_info.name);
        assert_eq!(info.ip, expeced_info.ip);
        assert_eq!(info.mac_address, expeced_info.mac_address);
    }

    #[test]
    fn test_bad_response() {
        /*
           Cache-Control: max-age=1800
           Server: WebOS/4.0.0 UPnP/1.0
           EXT:
           USN: uuid:ca7537ae-c393-4732-926a-29ac57738f04::urn:lge:device:tv:1
           ST: urn:lge:device:tv:1
           Date: Fri, 24 Feb 2023 01:19:31 GMT

        */
        let mut good_response = SearchResponse::new();
        good_response.set_raw("Cache-Control", vec![b"max-age=1800".to_vec()]);
        good_response.set_raw("Server", vec![b"WebOS/4.0.0 UPnP/1.0".to_vec()]);
        good_response.set_raw("EXT", vec![vec![]]);
        good_response.set_raw(
            "USN",
            vec![b"uuid:ca7537ae-c393-4732-926a-29ac57738f04::urn:lge:device:tv:1".to_vec()],
        );
        good_response.set_raw("ST", vec![b"urn:lge:device:tv:1".to_vec()]);

        good_response.set_raw("Date", vec![b"24 Feb 2023 01:19:31 GMT".to_vec()]);

        let ip = Ipv4Addr::new(192, 168, 0, 199);
        let address = SocketAddr::new(std::net::IpAddr::V4(ip), 1437);
        let info = parse_response(good_response, address);
        assert!(info.is_none());
    }
}
