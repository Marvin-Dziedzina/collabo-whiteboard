use bevy::prelude::*;
use common::TestMessage;
use lightyear::prelude::{AppMessageExt, NetworkDirection};

pub(crate) struct MessageProtocolPlugin;

impl Plugin for MessageProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<TestMessage>()
            .add_direction(NetworkDirection::ClientToServer);

        debug!("Messages registered");
    }
}

// app.add_message::<Message1>()
//     .add_direction(NetworkDirection::ServerToClient);
