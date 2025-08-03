use std::sync::mpsc;

pub struct Game {
    action: mpsc::Sender<Action>,
    update: mpsc::Receiver<Update>,
    data: Data,
}

pub struct Action;
pub struct Update;
struct Data;
