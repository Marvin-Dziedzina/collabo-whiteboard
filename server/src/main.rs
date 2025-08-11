use bevy::prelude::*;

use common::{CommonPlugin, LIGHTYEAR_TICKRATE};
use lightyear::prelude::server::ServerPlugins;
use protocol::ProtocolPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    app.add_plugins(ServerPlugins {
        tick_duration: LIGHTYEAR_TICKRATE,
    });

    app.add_plugins((CommonPlugin, ProtocolPlugin));

    app.run();
}
