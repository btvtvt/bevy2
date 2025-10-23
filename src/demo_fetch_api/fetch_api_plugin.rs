use bevy::app::{App, Plugin, Startup, Update};
use bevy::log::info;
use bevy::prelude::{Commands, Message, MessageReader, MessageWriter, Res, ResMut, Resource};
use bevy::tasks::AsyncComputeTaskPool;
use reqwest::StatusCode;
use std::io::Read;
use std::sync::Mutex;
use std::time::Duration;

pub struct FetchAPIPlugin;

#[derive(Message, Debug)]
struct FetchAPIMessage {
    // result: reqwest::Result<reqwest::blocking::Response>,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    body: String,
}

#[derive(Resource)]
struct FetchAPIState {
    client: reqwest::blocking::Client,
    // fetch_result: String,
    tx: async_std::channel::Sender<FetchAPIMessage>,
    rv: async_std::channel::Receiver<FetchAPIMessage>,
}

impl Plugin for FetchAPIPlugin {
    fn build(&self, app: &mut App) {
        let client = reqwest::blocking::Client::new();

        let (tx, rv) = async_std::channel::bounded::<FetchAPIMessage>(1);

        app.insert_resource(FetchAPIState {
            client,
            tx: tx.clone(),
            rv: rv.clone(),
        });

        app.add_message::<FetchAPIMessage>();

        app.add_systems(Startup, fetch_api_setup);
        app.add_systems(Update, (rv_fetch, fetch_check_message));
    }
}

fn fetch_api_setup(_commands: Commands, state: Res<FetchAPIState>) {
    info!("FetchAPIPlugin Startup");

    let pool = AsyncComputeTaskPool::get();

    let client = state.client.clone();
    let tx = state.tx.clone();

    let _task = pool
        .spawn(async move {
            info!("FetchAPIPlugin: fetch started");

            // async_std::task::sleep(Duration::from_secs(5)).await;

            let res = client
                .post("https://httpbin.org/post")
                .body("Hello from Rust + Bevy + Reqwest!")
                .send();

            info!("FetchAPIPlugin: fetch done");

            if let Ok(res) = res {
                // info!("FetchAPIPlugin: fetch result: {:#?}", res);

                let headers = res.headers().to_owned();

                tx.send(FetchAPIMessage {
                    status: res.status(),
                    headers,
                    body: res.text().unwrap(),
                })
                .await
                .unwrap();
            } else {
                // info!("FetchAPIPlugin: fetch result error: {:#?}", res);
            }
        })
        .detach();
}

fn rv_fetch(state: ResMut<FetchAPIState>, mut writer: MessageWriter<FetchAPIMessage>) {
    while let Ok(v) = state.rv.try_recv() {
        writer.write(v);
    }
}

fn fetch_check_message(mut messages: MessageReader<FetchAPIMessage>) {
    for message in messages.read() {
        info!("FetchAPIPlugin: fetch_check_message got = {:#?}", message);
    }
}
