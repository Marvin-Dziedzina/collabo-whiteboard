use bevy::prelude::*;

pub const LIGHTYEAR_TICKRATE: std::time::Duration = std::time::Duration::from_millis(1000 / 60);

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {}
}
