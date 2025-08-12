use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod channels;

pub const LIGHTYEAR_TICKRATE: std::time::Duration = std::time::Duration::from_millis(1000 / 60);
pub const SEND_INTERVALL: std::time::Duration = std::time::Duration::from_millis(1000 / 10);
pub const SERVER_ADDR: std::net::SocketAddr = std::net::SocketAddr::new(
    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    16888,
);
pub const CLIENT_ADDR: std::net::SocketAddr = std::net::SocketAddr::new(
    std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 2)),
    16777,
);

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestMessage(pub u128);
