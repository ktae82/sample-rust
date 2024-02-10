use std::io;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::StatusData;

pub struct SendWorker {
    mpsc_sender: Arc<Mutex<mpsc::Sender<StatusData>>>,
}

impl SendWorker {
    pub fn new(sender: Arc<Mutex<mpsc::Sender<StatusData>>>) -> Self {
        Self {
            mpsc_sender: sender,
        }
    }

    pub fn start(&mut self) {
        let mpsc_send_clone = self.mpsc_sender.clone();

        loop {
            println!("Enter command: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read");
            if input.trim().is_empty() {
                continue;
            }

            let input = input.trim().to_lowercase();

            // create data
            let mut status_data = StatusData::new();
            status_data.set_data(input.clone());

            // println!("Send - {}", status_data.get_data());
            mpsc_send_clone.lock().unwrap().send(status_data).unwrap();

            if input == "q" {
                break;
            }
        }

        println!("Finish SendWorker");
    }
}
