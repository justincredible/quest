use eframe::egui;

pub enum Game {
    Loaded(Option<GameData>),
    Exited,
}

impl Game {
    pub fn load(index: Option<usize>) -> Self {
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
    fn gui(&mut self, context: &egui::Context) {
	egui::CentralPanel::default().show(context, |ui| {
	    ui.label("oops...");
	    todo!();
	});
    }
}
