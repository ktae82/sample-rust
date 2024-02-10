use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::StatusData;

pub struct RecvWorker {
    mpsc_receiver: Arc<Mutex<mpsc::Receiver<StatusData>>>,
    thread_handle: Option<JoinHandle<()>>,
}

impl RecvWorker {
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<StatusData>>>) -> Self {
        Self {
            mpsc_receiver: receiver,
            thread_handle: None,
        }
    }

    pub fn start(&mut self) {
        let mpsc_receiver_clone = self.mpsc_receiver.clone();
        let mut is_quit = false;

        let handle = thread::spawn(move || {
            loop {
                if is_quit {
                    break;
                }

                match mpsc_receiver_clone.lock().unwrap().try_recv() {
                    Ok(status_data) => {
                        let received_data = status_data.get_data();
                        // println!("Recv status_data - {}", status_data.to_string());
                        println!("Recv - {}", received_data.clone());

                        if received_data == "q" {
                            is_quit = true;
                        }
                    }
                    Err(_) => {
                        // println!("there is no data");
                        continue;
                    }
                }

                thread::sleep(Duration::from_millis(10));
            }
        });

        self.thread_handle = Some(handle);
    }

    pub fn join(&mut self) {
        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
    }
}
