use bevy::prelude::*;
use common::channels::{OrderedReliableChannel, UnorderedReliableChannel};
use lightyear::prelude::{AppChannelExt, ChannelMode, ChannelSettings, ReliableSettings};

pub(crate) struct ChannelProtocolPlugin;

impl Plugin for ChannelProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_channel::<OrderedReliableChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..Default::default()
        });

        app.add_channel::<UnorderedReliableChannel>(ChannelSettings {
            mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
            ..Default::default()
        });
    }
}

// app.add_channel::<Channel1>(ChannelSettings {
//     mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
//     ..default()
// });
