use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Start");
    let (tx, rx) = channel();
    let rx = Arc::new(Mutex::new(rx));

    for i in 1..=10 {
        let rx = Arc::clone(&rx);
        tokio::spawn(async move {
            loop {
                let msg = match rx.lock().unwrap().try_recv() {
                    Ok(msg) => msg,
                    Err(_) => continue, //usar break para fechar thread quando terminarl de ler
                };
                println!("{} {}", i, msg);
            }
        });
    }

    for i in 0..=2 {
        let msg = format!("Message {}", i);
        tx.send(msg).unwrap();
    }

    println!("Done");

    thread::sleep(Duration::from_secs(2));
    for i in 0..=20 {
        let msg = format!("Message {}", i);
        tx.send(msg).unwrap();
    }
    loop{}
}