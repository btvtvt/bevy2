use bevy::prelude::*;
use bevy_server_client::{demo_fetch_api::fetch_api_plugin, demo_my_plugin::my_plugin};

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct ClientState {
    is_connected: bool,
}

fn main() {
    let mut app = bevy::app::App::new();

    // app.add_plugins(MinimalPlugins);
    app.add_plugins(DefaultPlugins);

    app.insert_resource(GreetTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    app.insert_resource(ClientState {
        is_connected: false,
    });

    // SYSTEMS
    // SETUP SYSTEMS
    app.add_systems(Startup, setup_camera_system);
    // app.add_systems(Startup, setup_gui_system);
    // // UPDATE SYSTEMS
    // app.add_systems(Update, update_client_gui_system);
    // app.add_systems(Update, receive_message_system);
    // app.add_systems(Update, send_message_system);
    // // app.add_systems(Update, disconnect_after_delay_system);
    // app.add_systems(Update, connect_and_disconnect_on_press_system);
    // app.add_systems(Update, client_join_leave_system);

    // CUSTOM PLUGINS
    app.add_plugins(my_plugin::MyPlugin);
    app.add_plugins(fetch_api_plugin::FetchAPIPlugin);
    // app.add_plugins(HeavyComputePlugin);

    app.run();
}

fn setup_camera_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}
