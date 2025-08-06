use eframe::egui;

use crate::Gui;
use super::Game;
use super::Minibuffer;

pub enum State {
    Loaded(Option<Game>),
    Exited,
}

impl Default for State {
    fn default() -> Self {
	State::Loaded(None)
    }
}

#[derive(Default)]
pub struct Menu {
    pub state: State,
    minibuffer: Minibuffer,
}

impl Menu {
    pub fn start(index: Option<usize>) -> Self {
	if let Some(_size) = index {
	    todo!();
	} else {
	    Menu::default()
	}
    }
}

impl Gui for Menu {
    fn gui(&mut self, context: &egui::Context) {
	egui::SidePanel::left("Menu").resizable(false).show(context, |ui| {
	    ui.with_layout(
		egui::Layout::bottom_up(egui::Align::Center)
		    .with_cross_justify(true),
		|ui| {
		    if ui.button("quit").clicked() {
			self.state = State::Exited;
		    }
		},
	    );
	});
	self.minibuffer.gui(context);
	if self.minibuffer.closing() {
	    self.state = State::Exited;
	}
        egui::CentralPanel::default().show(context, |ui| {
            match self {
                Menu { state: State::Loaded(None), .. } => {
		    let mut game = Game::new();
		    ui.label(game.output());
		    self.state = State::Loaded(Some(game));
                },
                Menu { state: State::Loaded(Some(game)), .. } => {
		    ui.label(game.output());
                },
                Menu { state: State::Exited, .. } => {
		    ui.label("Fare thee well!");
		},
            }
        });
    }
}
