use eframe::egui;

pub enum Game {
    Loaded(Option<crate::game::Quest>),
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

impl crate::app::Gui for Game {
    fn gui(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            match self {
                Game::Loaded(None) => {
                },
                Game::Loaded(Some(_game)) => {
                },
                Game::Exited => (),
            }
        });
    }
}

fn ui(ui: &mut egui::Ui) {

}
