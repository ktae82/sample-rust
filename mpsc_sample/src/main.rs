pub mod data;
pub mod receiver;
pub mod sender;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::data::status_data::StatusData;
use crate::receiver::recv_worker::RecvWorker;
use crate::sender::send_worker::SendWorker;

fn main() {
    let (mpsc_sender, mpsc_receiver) = mpsc::channel::<StatusData>();
    let mut recv_worker = RecvWorker::new(Arc::new(Mutex::new(mpsc_receiver)));
    recv_worker.start();

    let mut send_worker = SendWorker::new(Arc::new(Mutex::new(mpsc_sender)));
    send_worker.start();

    recv_worker.join();

    println!("Finished mpsc_sample main()");
}
