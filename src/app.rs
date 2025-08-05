use eframe::egui;

pub enum App {
    Menu(crate::Menu),
    Game(crate::game::Menu),
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
	match self {
	    App::Menu(menu) => match *menu {
		crate::Menu::Play(game) => {
		    let mut menu = crate::game::Menu::start(game);
		    menu.gui(context);
		    *self = App::Game(menu);
		},
		_ => menu.gui(context),
	    },
	    App::Game(game) => match game.state() {
		crate::game::State::Exited => {
		    let mut menu = crate::Menu::Start;
		    menu.gui(context);
		    *self = App::Menu(menu);
		},
		_ => game.gui(context),
	    },
	};
    }
}

pub trait Gui {
    fn gui(&mut self, _ctx: &egui::Context);
}
