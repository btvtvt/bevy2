use bevy::asset::AssetServer;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::text::TextFont;
use bevy::time::{Time, Timer, TimerMode};
use bevy::ui::PositionType;
use bevy::{text, DefaultPlugins};
use bevy_renet::netcode::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;
use bevy_server_client::demo_fetch_api::fetch_api_plugin::FetchAPIPlugin;
use bevy_server_client::demo_my_plugin::my_plugin::MyPlugin;
use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct ClientStateGUI;

#[derive(Resource)]
struct ClientState {
    is_connected: bool,
}

fn get_client_connect_addr() -> SocketAddr {
    let client_connect_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    client_connect_addr
}

fn create_renet_client(addr: SocketAddr) -> (NetcodeClientTransport, RenetClient) {
    let authentication = ClientAuthentication::Unsecure {
        server_addr: addr,
        client_id: 0,
        user_data: None,
        protocol_id: 0,
    };

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    let renet_conn_config = ConnectionConfig::default();
    let client = RenetClient::new(renet_conn_config);
    // client.disconnect();

    // return transport and client
    (transport, client)
}

fn main() {
    let mut app = bevy::app::App::new();

    // app.add_plugins(MinimalPlugins);
    app.add_plugins(DefaultPlugins);

    // Setup Renet Client
    app.add_plugins(RenetClientPlugin);
    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);

    // RENET STUFF
    let (transport, client) = create_renet_client(get_client_connect_addr());
    app.insert_resource(transport);
    app.insert_resource(client);

    // SELF RESOURCE
    app.insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
    app.insert_resource(ClientState {
        is_connected: false,
    });

    // SYSTEMS
    // SETUP SYSTEMS
    // app.add_systems(Startup, setup_camera_system);
    // app.add_systems(Startup, setup_gui_system);
    // // UPDATE SYSTEMS
    // app.add_systems(Update, update_client_gui_system);
    // app.add_systems(Update, receive_message_system);
    // app.add_systems(Update, send_message_system);
    // // app.add_systems(Update, disconnect_after_delay_system);
    // app.add_systems(Update, connect_and_disconnect_on_press_system);
    // app.add_systems(Update, client_join_leave_system);

    // CUSTOM PLUGINS
    app.add_plugins(MyPlugin);
    // app.add_plugins(HeavyComputePlugin);
    app.add_plugins(FetchAPIPlugin);

    app.run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_gui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // client_state: Res<ClientState>,
) {
    let font = asset_server.load("fonts/GoogleSansCode-Medium.ttf");

    let text_font = TextFont {
        font: font.clone(),
        font_size: 30.0,
        ..default()
    };

    commands.spawn((
        Text::new("Client Connected state: ?"),
        text_font.clone(),
        TextLayout::new_with_justify(text::Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(10),
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.5)),
        ClientStateGUI,
    ));
}

fn update_client_gui_system(
    mut commands: Commands,
    query: Query<Entity, (With<Text>, With<ClientStateGUI>)>,
    client_state: Res<ClientState>,
) {
    let text_entity = query.single().unwrap();

    commands.entity(text_entity).insert(Text::new(format!(
        "Client Connected state: {:?}",
        client_state.is_connected
    )));
}

fn connect_and_disconnect_on_press_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut client: ResMut<RenetClient>,
    // mut netcode: ResMut<NetcodeClientTransport>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyC) {
        if client.is_connected() {
            println!("Client already Connected!");
        } else {
            println!("Connect");

            let (transport, client) = create_renet_client(get_client_connect_addr());
            commands.insert_resource(transport);
            commands.insert_resource(client);
        }
    }

    if keys.just_pressed(KeyCode::KeyD) {
        client.disconnect();
        println!("Disconnect");
    }
}

// Systems
fn receive_message_system(mut client: ResMut<RenetClient>) {
    // println!("client id: {:#?}", client.);

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        // Handle received message
        println!("client got message: {:?}", message);
    }
}

fn send_message_system(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut client: ResMut<RenetClient>,
    mut client_state: ResMut<ClientState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if client.is_connected() {
            // println!("send_message_system: client connected");

            // if time.elapsed_secs() > 2.0 {
            //     client.send_message(
            //         DefaultChannel::ReliableOrdered,
            //         "client -> server message #2",
            //     );
            // } else {
            //     client.send_message(DefaultChannel::ReliableOrdered, "client message to server");
            // }

            client.send_message(DefaultChannel::ReliableOrdered, "client message to server");
        } else {
            // println!("send_message_system: client disconnected");
        }

        client_state.is_connected = client.is_connected();
    }
}

// fn disconnect_after_delay_system(
//     time: Res<Time>,
//     mut client: ResMut<RenetClient>,
//     mut dc: ResMut<DisconnectState>,
//     mut exit: MessageWriter<AppExit>,
// ) {
//     if time.elapsed_secs() > 100.0 {
//         if !dc.is_disconnected {
//             client.disconnect();
//             dc.is_disconnected = true;
//             println!("Client disconnected. and exiting");
//             exit.write(AppExit::Success);
//         }
//     }
// }

// fn client_join_leave_system(client: Res<RenetClient>) {
//     if client.is_connected() {
//         println!("client_join_leave_system: client connected");
//     } else {
//         println!("client_join_leave_system: client disconnected");
//     }
// }
