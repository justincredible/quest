use std::sync::mpsc;

pub struct Quest {
    channel: mpsc::Receiver<Update>,
    data: GameData,
}

enum Update {
}

pub struct GameData {
}

