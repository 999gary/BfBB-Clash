#![cfg_attr(
    all(target_os = "windows", not(feature = "console")),
    windows_subsystem = "windows"
)]

use std::{
    error::Error,
    sync::mpsc::{channel, Sender},
};

use clash::protocol::{Connection, Message, ProtocolError};
use log::{debug, error, info};
use tokio::{net::TcpStream, select};

mod dolphin;
mod game;
mod gui;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    env_logger::Builder::new()
        .format_level(true)
        .format_module_path(true)
        .format_target(false)
        .format_indent(Some(4))
        .format_timestamp_secs()
        .filter_level(log::LevelFilter::Debug)
        .parse_env("CLASH_LOG")
        .init();

    let (network_sender, network_receiver) = tokio::sync::mpsc::channel::<Message>(100);
    let (logic_sender, logic_receiver) = channel::<Message>();
    let (error_sender, error_receiver) = channel::<Box<dyn Error + Send>>();
    // Create a new thread and start a tokio runtime on it for talking to the server
    // TODO: Tokio may not be the best tool for the client. It might be better to
    //       simply use std's blocking networking in a new thread, since we should only ever
    //       have a single connection. Unfortunately for now we need to use it since the shared
    //       library is async.
    let _network_thread = std::thread::Builder::new()
        .name("Network".into())
        .spawn(move || start_network(network_receiver, logic_sender, error_sender));

    // Start Game Thread
    let (gui_sender, gui_receiver) = channel();
    let _game_thread = {
        let network_sender = network_sender.clone();
        std::thread::Builder::new()
            .name("Logic".into())
            .spawn(move || game::start_game(gui_sender, network_sender, logic_receiver))
    };

    // Start gui on the main thread
    gui::run(gui_receiver, error_receiver, network_sender);
}

fn load_ip_address() -> String {
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.push("ipaddress");
        if let Ok(ip) = std::fs::read_to_string(exe_path) {
            return ip;
        }
    }

    "127.0.0.1:42932".into()
}

#[tokio::main(flavor = "current_thread")]
async fn start_network(
    mut receiver: tokio::sync::mpsc::Receiver<Message>,
    logic_sender: Sender<Message>,
    error_sender: Sender<Box<dyn Error + Send>>,
) {
    let ip = load_ip_address();
    info!("Connecting to server at '{ip}'");

    let sock = TcpStream::connect(&ip).await.unwrap();
    let mut conn = Connection::new(sock);
    conn.write_frame(Message::Version {
        version: VERSION.to_owned(),
    })
    .await
    .unwrap();

    loop {
        select! {
            m = receiver.recv() => {
                debug!("Sending message {m:#?}");
                if let Err(e) = conn.write_frame(m.unwrap()).await {
                    error!("Error sending message to server. Disconnecting. {e:#?}");
                    error_sender.send(Box::new(e)).expect("GUI has crashed and so will we.");
                };
            }
            incoming = conn.read_frame() => {
                let incoming = match incoming {
                    Ok(Some(x)) => {
                        debug!("Received message {x:#?}.");
                        x
                    }
                    Ok(None) => {
                        info!("Server closed connection. Disconnecting.");
                        break;
                    }
                    Err(e) => {
                        error!("Error reading message from server. Disconnecting.\n{e}");
                        error_sender.send(e.into()).expect("GUI has crashed and so will we.");
                        break;
                    }
                };

                if let Err(e) = process_incoming(incoming, &mut conn, &logic_sender).await {
                    log::error!("Error from server:\n{e}");
                    error_sender.send(Box::new(e)).expect("GUI has crashed and so will we.");
                }
            }
        };
    }
}

async fn process_incoming(
    message: Message,
    _conn: &mut Connection,
    logic_sender: &Sender<Message>,
) -> Result<(), ProtocolError> {
    match message {
        Message::Version { version: _ } => {
            todo!()
        }
        Message::Error { error } => {
            return Err(error);
        }
        m @ Message::ConnectionAccept { player_id: _ } => {
            debug!("ConnectionAccept message got :)");
            logic_sender.send(m).unwrap();
        }
        Message::PlayerOptions { options: _ } => todo!(),
        Message::GameHost => todo!(),
        Message::GameJoin { lobby_id: _ } => todo!(),
        Message::GameOptions { options: _ } => todo!(),
        m @ Message::GameLobbyInfo { lobby: _ } => {
            logic_sender.send(m).unwrap();
        }
        m @ Message::GameBegin => {
            logic_sender.send(m).unwrap();
        }
        Message::GameCurrentRoom { room: _ } => todo!(),
        Message::GameForceWarp { room: _ } => todo!(),
        Message::GameItemCollected { item: _ } => todo!(),
        Message::GameEnd => {
            // This message isn't supposed to do anything until the GUI gets updated.
        }
        Message::GameLeave => todo!(),
    }

    Ok(())
}
