use bevy::prelude::*;

pub(crate) struct ChannelProtocolPlugin;

impl Plugin for ChannelProtocolPlugin {
    fn build(&self, app: &mut App) {}
}

// app.add_channel::<Channel1>(ChannelSettings {
//     mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
//     ..default()
// });
