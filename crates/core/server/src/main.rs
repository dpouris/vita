use std::sync::{Arc, mpsc::{self, Sender}};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use scheduler::time::MINUTE_IN_SECS;
use vita_socket::{
    client::SocketClient,
    connection::SocketConnection,
    error::SocketErrorKind,
    error_here,
    response::{SocketResponse, Status},
    signals::{Action, Signal},
};

fn main() {
    let mut sc = SocketClient::new("/tmp/vita.sock");
    sc.on_signal(Signal::CtrlC, Action::TerminateExit(0));

    match sc.bind() {
        Ok(_) => {
            println!("Waiting for connections...");
            let (termination_sender, termination_receiver) = mpsc::channel();
            let termination_sender = Arc::new(termination_sender);
            sc.on_recv(termination_receiver, Action::TerminateExit(0));

            for connection in sc.accept_connections() {
                let Ok(connection) = connection else { continue }; //TODO: handle err
                let tx = termination_sender.clone();
                connection.handle_par(move |conn| connection_handler(conn, tx));
            }
        }
        Err(bind_err) => {
            bind_err.log();
            if !matches!(bind_err.kind(), SocketErrorKind::SocketAddressInUse) {
                let Err(term_err) = SocketClient::terminate(sc.socket_path) else {
                    return;
                };
                term_err.log();
            }
        }
    };
}

fn connection_handler(mut conn: SocketConnection, tx: Arc<Sender<()>>) {
    println!("[Handling connection]");

    loop {
        // --- Read Request ---
        let Ok(msg) = conn.block().recv_msg() else {
            println!("[Closing connection]");
            return;
        };

        println!("Received: {msg:?}");

        // --- Terminate Socket ---
        msg.is_break().then(|| tx.send(()));

        // --- Terminate Connection ---
        if msg.is_stop() {
            println!("[Closing connection]");
            break;
        }

        // --- Respond ---
        let response = SocketResponse::new(Status::Accepted);
        let Ok(_) = conn.block().post_reply(response).map_err(|err| {
            err.err_where(error_here!()).log();
        }) else {
            return;
        };
    }
}
