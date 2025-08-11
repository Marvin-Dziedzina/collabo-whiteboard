use bevy::prelude::*;
use lightyear::prelude::client::*;

use common::{CommonPlugin, LIGHTYEAR_TICKRATE};
use protocol::ProtocolPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(ClientPlugins {
        tick_duration: LIGHTYEAR_TICKRATE,
    });

    app.add_plugins((CommonPlugin, ProtocolPlugin));

    app.run();
}
