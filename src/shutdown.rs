use futures::StreamExt;
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;
use tokio::sync::{oneshot, oneshot::Receiver, oneshot::Sender};

pub fn register() -> Receiver<()>{
    let signals = Signals::new([SIGHUP, SIGINT, SIGQUIT, SIGTERM]).unwrap();
    signals.handle();
    let (tx, rx) : (Sender<()>, Receiver<()>) = oneshot::channel();
    tokio::spawn(handle_signal(signals, tx));
    rx
}

async fn handle_signal(mut signal: Signals, tx: oneshot::Sender<()>){
    while let Some(signal) = signal.next().await {
        match signal {
            SIGHUP => {

            }

            SIGTERM | SIGINT | SIGQUIT => {
                let _ = tx.send(());
                return;
            }

            _ => unreachable!(),
        }
    }
}