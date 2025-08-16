use bevy::prelude::*;
use lightyear::prelude::{AppMessageExt, NetworkDirection};

pub(crate) struct MessageProtocolPlugin;

impl Plugin for MessageProtocolPlugin {
    fn build(&self, app: &mut App) {
        debug!("Messages registered");
    }
}

// app.add_message::<Message1>()
//     .add_direction(NetworkDirection::ServerToClient);
