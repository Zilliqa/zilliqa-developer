use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use notify::{recommended_watcher, RecursiveMode, Watcher};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    process::{Child, Command},
    sync::broadcast,
    time::{sleep, Duration},
};

async fn stream_output(mut child: Child) -> Result<Child, Box<dyn std::error::Error>> {
    let mut stdout = child
        .stdout
        .take()
        .ok_or("Child process did not have a stdout")?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or("Child process did not have a stderr")?;

    let _stdout_handle = tokio::spawn(async move {
        let mut buf = vec![0u8; 1024];
        loop {
            match stdout.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    io::stdout()
                        .write_all(&buf[..n])
                        .await
                        .expect("Failed to write to stdout");
                }
                Err(e) => eprintln!("Error reading from stdout: {:?}", e),
            }
        }
    });

    let _stderr_handle = tokio::spawn(async move {
        let mut buf = vec![0u8; 1024];
        loop {
            match stderr.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    io::stderr()
                        .write_all(&buf[..n])
                        .await
                        .expect("Failed to write to stderr");
                }
                Err(e) => eprintln!("Error reading from stderr: {:?}", e),
            }
        }
    });

    // tokio::try_join!(stdout_handle, stderr_handle)?;
    Ok(child)
}

async fn run_command(command: &str, args: &[&str]) -> Result<Child, Box<dyn std::error::Error>> {
    let child = Command::new(command)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    Ok(child)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (send, mut recv) = broadcast::channel::<()>(24);
    let send = Arc::new(Mutex::new(send));

    std::thread::spawn(move || {
        let mut watcher = recommended_watcher(move |res| match res {
            Ok(event) => {
                println!("event: {:?}", event);
                send.lock().unwrap().send(()).unwrap();
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        })
        .unwrap();
        watcher
            .watch(Path::new("."), RecursiveMode::Recursive)
            .unwrap();
    });

    loop {
        sleep(Duration::from_secs(1)).await;

        let trunk = run_command("sleep", &["10"]).await?;
        let cargo_watch = run_command("sleep", &["100"]).await?;

        let mut trunk_handle = stream_output(trunk).await?;
        let mut cargo_watch_handle = stream_output(cargo_watch).await?;
        sleep(Duration::from_secs(1)).await;

        tokio::select! {
            _ = trunk_handle.wait() => {
                println!("Trunk stopped.");
                let _ = trunk_handle.kill();
                let _ = cargo_watch_handle.kill();
                break;

            },
            _ = cargo_watch_handle.wait() => {
                println!("Cargo stopped.");
                let _ = trunk_handle.kill();
                let _ = cargo_watch_handle.kill();
                break;
            },

            _ = recv.recv() => {
                println!("Refreshing.");
                let _ = trunk_handle.kill();
                let _ = cargo_watch_handle.kill();
                break;
                // If a change is detected, break out of the select! macro to kill and restart the processes.
            }

        }
    }
    Ok(())
}
