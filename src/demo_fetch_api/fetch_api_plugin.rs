use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
// use reqwest::StatusCode;
// use std::io::Read;
// use std::sync::Mutex;
// use std::time::Duration;

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
    client: reqwest::Client,
    // fetch_result: String,
    tx: async_std::channel::Sender<FetchAPIMessage>,
    rv: async_std::channel::Receiver<FetchAPIMessage>,
}

impl Plugin for FetchAPIPlugin {
    fn build(&self, app: &mut App) {
        let client = reqwest::Client::new();

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
                .send()
                .await;

            info!("FetchAPIPlugin: fetch done");

            if let Ok(res) = res {
                info!("FetchAPIPlugin: fetch result: \n {:#?}", res);

                let resp_status = res.status();
                let resp_headers = res.headers().to_owned();
                let resp_text = res.text().await.unwrap();

                tx.send(FetchAPIMessage {
                    status: resp_status,
                    headers: resp_headers,
                    body: resp_text,
                })
                .await
                .unwrap();
            } else if let Err(err) = res {
                info!("FetchAPIPlugin: fetch result error: {:#?}", err);
            }
        })
        .detach();
}

fn rv_fetch(state: ResMut<FetchAPIState>, mut fetch_api_writer: MessageWriter<FetchAPIMessage>) {
    while let Ok(v) = state.rv.try_recv() {
        fetch_api_writer.write(v);
    }
}

fn fetch_check_message(mut fetch_api_messages: MessageReader<FetchAPIMessage>) {
    for message in fetch_api_messages.read() {
        info!("FetchAPIPlugin: fetch_check_message = \n{:#?}", message);
    }
}
