use bevy::{log::LogPlugin, prelude::*};

use common::{CommonPlugin, LIGHTYEAR_TICKRATE, SEND_INTERVALL, SERVER_ADDR, TestMessage};
use lightyear::{
    netcode::NetcodeServer,
    prelude::{
        server::{NetcodeConfig, ServerPlugins, ServerUdpIo, Start},
        *,
    },
};
use protocol::ProtocolPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((MinimalPlugins, LogPlugin::default()));

    app.add_plugins(ServerPlugins {
        tick_duration: LIGHTYEAR_TICKRATE,
    });

    app.add_plugins((ProtocolPlugin, CommonPlugin));

    app.add_observer(on_client_connect);

    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, message_receiver);

    app.run();
}

fn setup(mut commands: Commands) {
    let server = commands
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(SERVER_ADDR),
            ServerUdpIo::default(),
        ))
        .id();

    commands.trigger_targets(Start, server);
}

fn on_client_connect(trigger: Trigger<OnAdd, LinkOf>, mut commands: Commands) {
    commands.entity(trigger.target()).insert((
        ReplicationSender::new(SEND_INTERVALL, SendUpdatesMode::SinceLastAck, false),
        Name::from("Client"),
    ));
}

fn message_receiver(mut message_receiver: Single<&mut MessageReceiver<TestMessage>>) {
    for msg in message_receiver.receive() {
        info!("Message: {}", msg.0);
    }
}
