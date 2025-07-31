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
	egui::SidePanel::left("Menu").resizable(false).show(context, |ui| {
	    let layout = egui::Layout {
		main_dir: egui::Direction::BottomUp,
		cross_align: egui::Align::Center,
		cross_justify: true,
		..Default::default()
	    };
	    ui.with_layout(layout, |ui| {
		if ui.button("quit").clicked() {
		    *self = Game::Exited;
		}
	    });
	});
	egui::TopBottomPanel::bottom("Minibuffer").resizable(false).show(context, |ui| {
	});
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
