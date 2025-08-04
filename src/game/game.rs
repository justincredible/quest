use std::sync::mpsc;

pub struct Game {
    action: mpsc::Sender<Action>,
    update: mpsc::Receiver<Update>,
    data: Data,
}

pub enum Action {
    Start,
}

pub enum Update {
    New(String),
}

struct Data;
