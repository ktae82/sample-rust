use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::StatusDB;

pub struct SendWorker {
    mpsc_send: Arc<Mutex<mpsc::Sender<StatusDB>>>,
}

impl SendWorker {
    pub fn new(send: Arc<Mutex<mpsc::Sender<StatusDB>>>) -> Self {
        Self { mpsc_send: send }
    }

    pub fn start(&self) {
        let mpsc_send_clone = self.mpsc_send.clone();
        let mut index = 0;

        thread::spawn(move || loop {
            let mut status_db = StatusDB::new();
            status_db.set_data(index);
            mpsc_send_clone.lock().unwrap().send(status_db).unwrap();

            thread::sleep(Duration::from_secs(1));
            index += 10;
        });
    }
}
