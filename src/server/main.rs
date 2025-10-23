use bevy::app::Startup;
use bevy::log::info;
use bevy::prelude::{MessageReader, ResMut, Update};
use bevy::MinimalPlugins;
use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;
use renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

fn main() {
    let server_addr: SocketAddr = "0.0.0.0:5000".parse().unwrap();

    let mut app = bevy::app::App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(bevy::log::LogPlugin::default());

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);

    let socket = UdpSocket::bind(server_addr).unwrap();

    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);

    // app.add_systems(send_message_system);
    app.add_systems(Startup, hello_world);
    app.add_systems(Update, receive_message_system);
    app.add_systems(Update, handle_server_events_system);

    // app.add_systems(Update, send_message_system);

    app.run();
}

// Systems

fn hello_world() {
    info!("server/main.rs hello world! server started");
}

// fn send_message_system(mut server: ResMut<RenetServer>) {
//     // let channel_id = 0;
//     // Send a text message for all clients
//     // The enum DefaultChannel describe the channels used by the default configuration
//     server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
// }

fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            // Handle received message

            // info!("client [{}] received message: {:?}", client_id, message);

            info!("client [{}] received message: {:?}", client_id, message);
        }

        if !server.can_send_message(client_id, 1, 1usize) {
            info!(
                "client [{}] can't send message. disconnecting this client",
                client_id
            );

            server.disconnect(client_id);
        }
    }
}

fn handle_server_events_system(mut server_events: MessageReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client {client_id} connected");
            }

            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
