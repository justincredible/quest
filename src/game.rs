use eframe::egui;

pub enum Game {
    Loaded(Option<GameData>),
    Exited,
}

impl Game {
    pub fn load(index: Option<usize>) -> Game {
	if let Some(_size) = index {
	    todo!();
	} else {
	    Game::Loaded(None)
	}
    }
}

pub struct GameData {
}

impl crate::app::Gui for Game {
    fn gui(&mut self, ui: &mut egui::Ui) {
	ui.label("oops...");
	todo!();
    }
}
