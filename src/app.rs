use eframe::egui;

pub enum App {
    Menu(crate::Menu),
    Game(crate::Game),
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
	match self {
	    App::Menu(menu_type) => match *menu_type {
		crate::Menu::Play(game) => *self = App::Game(crate::Game::load(game)),
		_ => { egui::CentralPanel::default().show(context, |ui| menu_type.gui(ui)); },
	    },
	    App::Game(game_type) => match game_type {
		crate::Game::Exited => *self = App::Menu(crate::Menu::Start),
		_ => { egui::CentralPanel::default().show(context, |ui| game_type.gui(ui)); },
	    },
	};
    }
}

pub trait Gui {
    fn gui(&mut self, _ui: &mut egui::Ui);
}
