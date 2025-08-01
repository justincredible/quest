use eframe::egui;

pub enum State {
    Loaded(Option<crate::game::Quest>),
    Exited,
}

pub struct Menu {
    state: State,
    minibuffer: crate::game::Minibuffer,
}

impl Menu {
    pub fn start(index: Option<usize>) -> Self {
	if let Some(_size) = index {
	    todo!();
	} else {
	    Menu {
		state: State::Loaded(None),
		minibuffer: Default::default(),
	    }
	}
    }

    pub fn state(&self) -> &State {
	&self.state
    }
}

impl crate::app::Gui for Menu {
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
            match &self.state {
                State::Loaded(None) => {
                },
                State::Loaded(Some(_game)) => {
                },
                State::Exited => (),
            }
        });
    }
}
