use bevy::prelude::*;
use lightyear::{
    netcode::{Key, NetcodeClient},
    prelude::{
        client::{ClientPlugins, NetcodeConfig},
        *,
    },
};

use common::{CLIENT_ADDR, CommonPlugin, LIGHTYEAR_TICKRATE, SERVER_ADDR, TestMessage};
use protocol::{PROTOCOL_ID, ProtocolPlugin, channel::default::UnorderedReliableChannel};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_plugins(ClientPlugins {
        tick_duration: LIGHTYEAR_TICKRATE,
    });

    app.add_plugins((ProtocolPlugin, CommonPlugin));

    app.add_systems(Startup, (setup, dump_channels))
        .add_systems(FixedUpdate, message_sender);

    app.run();
}

fn setup(mut commands: Commands) -> Result {
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: PROTOCOL_ID,
    };
    let client = commands
        .spawn((
            Client::default(),
            LocalAddr(CLIENT_ADDR),
            PeerAddr(SERVER_ADDR),
            Link::new(None),
            ReplicationReceiver::default(),
            MessageManager::default(),
            MessageSender::<TestMessage>::default(),
            NetcodeClient::new(auth, NetcodeConfig::default())?,
            UdpIo::default(),
        ))
        .id();

    commands.trigger_targets(Connect, client);

    Ok(())
}

fn message_sender(
    time: Res<Time>,
    mut message_sender: Single<&mut MessageSender<TestMessage>, With<Connected>>,
) {
    message_sender.send::<UnorderedReliableChannel>(TestMessage(time.elapsed().as_millis()));
}

fn dump_channels(reg: Res<ChannelRegistry>) {
    info!("Channel Registry: {:?}", reg.kind_map());
}
