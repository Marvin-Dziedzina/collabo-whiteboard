use bevy::prelude::*;

mod channel;
mod components;
mod inputs;
mod message;

use channel::ChannelProtocolPlugin;
use components::ComponentsProtocolPlugin;
use inputs::InputProtocolPlugin;
use message::MessageProtocolPlugin;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputProtocolPlugin,
            MessageProtocolPlugin,
            ComponentsProtocolPlugin,
            ChannelProtocolPlugin,
        ));
    }
}
