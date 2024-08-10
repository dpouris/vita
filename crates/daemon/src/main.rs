use daemon::start_daemon;
mod daemon;

fn main() {
    match start_daemon() {
        Ok(_) => {
            println!("successfully started daemon...");
        }
        Err(msg) => {
            if let Some(msg) = msg {
                eprintln!("{msg}")
            };
            return;
        }
    }

    // daemon logic
    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("YEah!");
        counter += 1;
    }
}
