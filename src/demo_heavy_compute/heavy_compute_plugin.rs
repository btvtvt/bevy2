use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::info;
use bevy::prelude::{Commands, ResMut, Resource};
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct HeavyComputePlugin;

#[derive(Resource)]
struct HeavyComputeState {
    // counter: Arc<AtomicU32>,
    counter: Arc<Mutex<u32>>,
    task: Option<Task<()>>,
}

impl Plugin for HeavyComputePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HeavyComputeState {
            // counter: Arc::new(AtomicU32::new(0)),
            counter: Arc::new(Mutex::new(0)),
            task: None,
        });

        app.add_systems(Startup, heavy_compute_plugin_setup);
        app.add_systems(Update, check_task);
    }
}

fn heavy_compute_plugin_setup(_commands: Commands, mut state: ResMut<HeavyComputeState>) {
    info!("HeavyComputePlugin Startup");

    let pool = AsyncComputeTaskPool::get();

    let counter_arc = state.counter.clone();

    let task = pool.spawn(async move {
        info!("HeavyComputePlugin: heavy compute task started");

        for _i in 1..5 {
            // counter_arc.fetch_add(1, Ordering::SeqCst);

            {
                let mut counter_lock = counter_arc.lock().unwrap();
                *counter_lock += 1;
            }

            async_std::task::sleep(Duration::from_millis(1000)).await;
        }

        info!("HeavyComputePlugin: heavy compute task done");
    });

    state.task = Some(task);
}

fn check_task(mut state: ResMut<HeavyComputeState>) {
    if let Some(task) = &mut state.task {
        if task.is_finished() {
            // info!("result: {}", state.counter.load(Ordering::SeqCst));
            info!(
                "HeavyComputePlugin result: {}",
                *state.counter.lock().unwrap()
            );

            state.task = None; // Remove completed task
        }
    }
}
