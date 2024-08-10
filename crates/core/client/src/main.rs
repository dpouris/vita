use std::process::exit;

use vita_socket::{
    client::SocketClient,
    connection::SocketConnection,
    error_here,
    request::{Message, SocketRequest},
};

fn main() {
    let sc = SocketClient::new("/tmp/vita.sock");
    let message = parse_args(std::env::args());
    let request = SocketRequest::new(message);

    match sc.connect().map_err(|err| err.err_where(error_here!())) {
        Ok(mut connection) => loop {
            if send_request(&mut connection, request).is_err() {
                eprintln!("Connection closed. Exiting...");
                return;
            };
            std::thread::sleep(std::time::Duration::from_secs(3));
        },
        Err(err) => err.log(),
    }
}

fn send_request(conn: &mut SocketConnection, req: SocketRequest) -> Result<(), ()> {
    let Ok(reply) = conn.block().post_msg(req) else {
        eprintln!("Couldn't read response");
        return Err(());
    };
    println!("Received: {:?}", reply);
    Ok(())
}

fn parse_args(mut args: std::env::Args) -> Message {
    let program = args.next().unwrap();
    let usage_str = format!("Usage: {} <command> [args...]", program);

    let Some(command) = args.next() else {
        eprintln!("{usage_str}");
        exit(1);
    };

    match command.as_str() {
        "break" => Message::Break,
        "stop" => Message::Stop,
        "forward" => Message::Forward,
        _ => Message::Unknown,
    }
}
