use std::sync::mpsc;

pub struct Game {
    action: mpsc::Sender<Action>,
    update: mpsc::Receiver<Update>,
}

struct Action;
struct Update;
