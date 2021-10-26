use std::time::Duration;

/// This component marks if an entity should respawn after being eliminated
#[derive(Copy, Clone, Debug)]
pub struct Respawn {
    pub respawn_duration: Duration,
}
