use std::sync::mpsc;

pub struct Quest {
    action: mpsc::Receiver<super::Action>,
    update: mpsc::Sender<super::Update>,
    state: State,
}

struct State;
