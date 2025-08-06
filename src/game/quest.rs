use std::sync::mpsc;

use super::Action;
use super::Update;

pub struct Quest {
    action: mpsc::Receiver<Action>,
    update: mpsc::Sender<Update>,
    state: State,
}

impl Quest {
    pub fn new(action: mpsc::Receiver<Action>, update: mpsc::Sender<Update>) -> Self {
	Self {
	    action,
	    update,
	    state: State,
	}
    }

    pub fn run(&self) {
	self.action.recv().map_or_else(
	    |error| eprintln!("{:?}", error),
	    |action| {
		match action {
		    Action::Start => self.update
			.send(Update::New("Welcome to QUEST!".to_string()))
			.expect("a new game"),
		    Action::ShutDown => return,
		}
		self.run();
	    },
	);
    }
}

struct State;
