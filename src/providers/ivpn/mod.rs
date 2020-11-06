mod openvpn;
mod wireguard;

use super::{ConfigurationChoice, OpenVpnProvider, Provider, WireguardProvider};
use crate::vpn::Protocol;

pub struct IVPN {}

impl Provider for IVPN {
    fn alias(&self) -> String {
        "ivpn".to_string()
    }

    fn default_protocol(&self) -> Protocol {
        Protocol::Wireguard
    }
}
