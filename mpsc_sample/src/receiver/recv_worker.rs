use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::StatusDB;

pub struct RecvWorker {
    mpsc_recv: Arc<Mutex<mpsc::Receiver<StatusDB>>>,
}

impl RecvWorker {
    pub fn new(recv: Arc<Mutex<mpsc::Receiver<StatusDB>>>) -> Self {
        Self { mpsc_recv: recv }
    }

    pub fn start(&self) {
        let mpsc_recv_clone = self.mpsc_recv.clone();

        thread::spawn(move || loop {
            match mpsc_recv_clone.lock().unwrap().try_recv() {
                Ok(status) => println!("Received: {}", status.get_data()),
                Err(_) => {
                    // println!("there is no data");
                    continue;
                }
            }

            thread::sleep(Duration::from_millis(10));
        });
    }
}
