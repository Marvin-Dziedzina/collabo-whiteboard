use bevy::prelude::*;
use lightyear::prelude::{
    AppChannelExt, ChannelMode, ChannelSettings, NetworkDirection, ReliableSettings,
};

use default::{OrderedReliableChannel, UnorderedReliableChannel};

pub mod default;

pub(crate) struct ChannelProtocolPlugin;

impl Plugin for ChannelProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_channel::<OrderedReliableChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..Default::default()
        })
        .add_direction(NetworkDirection::Bidirectional);

        app.add_channel::<UnorderedReliableChannel>(ChannelSettings {
            mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
            ..Default::default()
        })
        .add_direction(NetworkDirection::Bidirectional);

        debug!("Channels registered");
    }
}

// app.add_channel::<Channel1>(ChannelSettings {
//     mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
//     ..default()
// });
