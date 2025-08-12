use bevy::prelude::*;

mod channel;
mod components;
mod inputs;
mod message;

use channel::ChannelProtocolPlugin;
use components::ComponentsProtocolPlugin;
use inputs::InputProtocolPlugin;
use message::MessageProtocolPlugin;

/// **!!! INCREMENT WHEN THE PROTOCOL IS CHANGED !!!**
///
/// If a server and client have the same [`PROTOCOL_ID`] they can communicate. If not they can't.
pub const PROTOCOL_ID: u64 = 0;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChannelProtocolPlugin,
            MessageProtocolPlugin,
            ComponentsProtocolPlugin,
            InputProtocolPlugin,
        ));
    }
}
