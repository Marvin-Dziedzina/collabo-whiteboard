use bevy::prelude::*;

pub(crate) struct MessageProtocolPlugin;

impl Plugin for MessageProtocolPlugin {
    fn build(&self, app: &mut App) {}
}

// app.add_message::<Message1>()
//     .add_direction(NetworkDirection::ServerToClient);
