use eframe::egui;

pub enum State {
    Loaded(Option<crate::game::Quest>),
    Exited,
}

pub struct Game {
    state: State,
    minibuffer: Minibuffer,
}

impl Game {
    pub fn load(index: Option<usize>) -> Self {
	if let Some(_size) = index {
	    todo!();
	} else {
	    Game {
		state: State::Loaded(None),
		minibuffer: Default::default(),
	    }
	}
    }

    pub fn state(&self) -> &State {
	&self.state
    }
}

impl crate::app::Gui for Game {
    fn gui(&mut self, context: &egui::Context) {
	egui::SidePanel::left("Menu").resizable(false).show(context, |ui| {
	    ui.with_layout(vertical_layout(egui::Align::Center, true, egui::Direction::BottomUp), |ui| {
		if ui.button("quit").clicked() {
		    self.state = State::Exited;
		}
	    });
	});
	egui::TopBottomPanel::bottom("Minibuffer").resizable(false).show(context, |ui| {
	    ui.with_layout(
		vertical_layout(egui::Align::Center, true, egui::Direction::BottomUp), |ui| {
		    let minibuffer = ui.text_edit_singleline(&mut self.minibuffer.input);
		    if minibuffer.lost_focus() {
			self.minibuffer.execute();
			self.minibuffer.input.clear();
			if self.minibuffer.closing {
			    self.state = State::Exited;
			}
		    }
		    ui.label(&self.minibuffer.output);
		},
	    );
	});
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

#[derive(Default)]
struct Minibuffer {
    input: String,
    output: String,
    closing: bool,
}

impl Minibuffer {
    pub fn execute(&mut self) {
	let input = self.input.to_lowercase();

	if input.is_empty() {
	    self.output.clear();
	} else if input.as_str() == "quit" {
	    self.output = "Quitting...".to_string();
	    self.closing = true;
	} else {
	    self.output = format!("I don't understand \"{}\"", self.input);
	}
    }
}

fn vertical_layout(alignment: egui::Align, justified: bool, direction: egui::Direction) -> egui::Layout {
    egui::Layout {
	main_dir: direction,
	cross_align: alignment,
	cross_justify: justified,
	..Default::default()
    }
}
