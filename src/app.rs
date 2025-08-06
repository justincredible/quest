use eframe::egui;

use crate::Gui;
use crate::Menu;
use crate::game::Menu as Game;
use crate::game::State;

pub enum App {
    Menu(Menu),
    Game(Game),
}

impl Gui for App {
    fn gui(&mut self, context: &egui::Context) {
	match self {
	    App::Menu(menu) => menu.gui(context),
	    App::Game(game) => game.gui(context),
	}
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
	self.gui(context);
	match self {
	    App::Menu(Menu::Play(game)) => *self = App::Game(Game::start(*game)),
	    App::Game(Game { state: State::Exited, .. }) => *self = App::Menu(Menu::Start),
	    _ => (),
	};
    }
}
