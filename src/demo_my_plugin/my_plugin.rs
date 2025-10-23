use bevy::app::{App, Plugin, Startup};
use bevy::log::info;
use bevy::prelude::{Commands, Res, ResMut, Resource, Time, Timer, Update};
use bevy::time::TimerMode;

pub struct MyPlugin;

#[derive(Resource)]
struct Tick1S(Timer);

impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, my_plugin_setup);
        app.add_systems(Update, tick_1s_system);

        app.insert_resource(Tick1S(Timer::from_seconds(1.0, TimerMode::Repeating)));
    }
}

fn my_plugin_setup(mut commands: Commands) {
    info!("my_plugin_setup Startup");
}

fn tick_1s_system(time: Res<Time>, mut timer: ResMut<Tick1S>) {
    if timer.0.tick(time.delta()).just_finished() {
        let sec_elapsed = time.elapsed_secs_wrapped();
        info!("seconds since game started elapsed: {}", sec_elapsed);
    }
}
