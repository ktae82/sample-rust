pub mod data;
pub mod receiver;
pub mod sender;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::data::status_db::StatusDB;
use crate::receiver::recv_worker::RecvWorker;
use crate::sender::send_worker::SendWorker;

fn main() {
    let (sender, receiver) = mpsc::channel::<StatusDB>();
    let recv = RecvWorker::new(Arc::new(Mutex::new(receiver)));
    recv.start();

    let send = SendWorker::new(Arc::new(Mutex::new(sender)));
    send.start();

    thread::sleep(Duration::from_secs(10));
}
