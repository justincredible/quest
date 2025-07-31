use eframe::egui;

pub enum App {
    Menu(crate::Menu),
    Game(crate::Game),
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
	match self {
	    App::Menu(menu) => match *menu {
		crate::Menu::Play(game) => *self = App::Game(crate::Game::load(game)),
		_ => menu.gui(context),
	    },
	    App::Game(game) => match game.state() {
		crate::game::State::Exited => *self = App::Menu(crate::Menu::Start),
		_ => game.gui(context),
	    },
	};
    }
}

pub trait Gui {
    fn gui(&mut self, _ctx: &egui::Context);
}
