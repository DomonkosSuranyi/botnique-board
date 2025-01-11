use blaminar::simulation::LaminarConfig;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

pub mod collision;
pub mod components;
pub mod entities;
pub mod events;
pub mod metric_dimension;
pub mod network;
pub mod resources;
pub mod serialization;
pub mod systems;
pub mod utilities;

/// The move direction relative to facing
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum MoveDirection {
    Forward,
    Backward,
    RunForward,
    StrafeLeft,
    StrafeRight,
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    hartbeat_interval: u8,
}

impl From<NetworkConfig> for LaminarConfig {
    fn from(net: NetworkConfig) -> LaminarConfig {
        LaminarConfig {
            heartbeat_interval: Some(Duration::from_secs(net.hartbeat_interval as u64)),
            ..Default::default()
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerName(pub String);

impl fmt::Display for PlayerName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}
