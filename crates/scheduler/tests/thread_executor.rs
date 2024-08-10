use std::{env, thread};
use std::fs::File;
use std::future::Future;
use std::process::{Command, Output, Stdio};
use std::time::Duration;

#[test]
fn test_thread_executor() {
    let mut handles = vec![];

    let start = std::time::Instant::now();
    for _ in 0..500 {
        let handle = thread::spawn(move || {
            run_command()
        });
        handles.push(handle);
    }
    println!(
        "It took {} micros to spawn threads",
        start.elapsed().as_micros()
    );

    let start = std::time::Instant::now();
    for handle in handles {
        let _out = handle.join().unwrap().unwrap();
    }
    println!(
        "It took {} μs to complete work",
        start.elapsed().as_micros() - Duration::from_secs(5).as_micros()
    );
}

fn run_command() -> Option<Output> {
    // println!("Running command...");
    let cwd = env::current_dir().unwrap();
    let script_path = cwd.join("tests/test_script.sh");
    let logs = File::options()
        .read(true)
        .append(true)
        .open(cwd.join("tests/logs.txt"))
        .unwrap();

    // println!("File path is: {script_path:?}");
    let handle = Command::new(script_path).stdout(Stdio::from(logs)).spawn();

    match handle {
        Ok(mut handle) => handle.wait_with_output().ok(),
        Err(_) => None,
    }
}

// 1
// 2
