use std::sync::mpsc;
use std::thread;

pub struct Game {
    action: mpsc::Sender<Action>,
    update: mpsc::Receiver<Update>,
    data: Data,
    quest: thread::JoinHandle<()>
}

impl Game {
    pub fn new() -> Self {
	let (action, receiver) = mpsc::channel();
	let (sender, update) = mpsc::channel();
	let quest = thread::Builder::new().spawn(|| {
	    let quest = super::Quest::new(receiver, sender);
	    quest.run();
	}).expect("an OS thread");
	action.send(Action::Start).expect("a prompt response");
	Self {
	    action,
	    update,
	    data: Default::default(),
	    quest,
	}
    }

    pub fn output(&mut self) -> &str {
	if let Ok(update) = self.update.try_recv() {
	    match update {
		Update::New(string) => self.data.output = string,
	    }
	}
	&self.data.output
    }
}

impl Drop for Game {
    fn drop(&mut self) {
	self.action.send(Action::ShutDown).expect("a clean shutdown");
    }
}

pub enum Action {
    Start,
    ShutDown,
}

pub enum Update {
    New(String),
}

#[derive(Default)]
struct Data {
    output: String,
}
