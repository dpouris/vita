// use std::process::{Command, Output};

use std::env;
use std::future::Future;
use std::path::Path;
use std::process::Output;
use std::str::from_utf8;
use std::time::{Duration, Instant};

use tokio::process::Command;

#[tokio::test]
async fn test_async_executor() {
    let mut handles = vec![];

    let start = Instant::now();
    for _ in 0..500 {
        let handle = tokio::spawn(async move {
            run_command().await
        });

        handles.push(handle);
    }
    println!(
        "It took {} micros to spawn tasks",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    for (idx, handle) in handles.into_iter().enumerate() {
        let out = handle.await.unwrap().unwrap().stdout;
        let out = from_utf8(&out).unwrap();

        // println!("Output for {idx} is: {out}");
    }
    println!(
        "It took {} μs to complete work",
        start.elapsed().as_micros() - Duration::from_secs(5).as_micros()
    );
}

async fn run_command() -> Option<Output> {
    // println!("Running command...");
    let mut script_path = env::current_dir().unwrap();
    script_path.push(Path::new("tests/test_script.sh"));
    // println!("File path is: {script_path:?}");
    let handle = Command::new(script_path).output().await;
    handle.ok()
}