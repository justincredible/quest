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
	self.minibuffer.gui(context);
	if self.minibuffer.closing {
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

fn vertical_layout(cross_align: egui::Align, cross_justify: bool, direction: egui::Direction) -> egui::Layout {
    egui::Layout {
	main_dir: direction,
	cross_align,
	cross_justify,
	..Default::default()
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
	let inputs: Vec<&str> = input.split(|c: char| c.is_ascii_punctuation() || c.is_whitespace()).collect();

	if input.is_empty() {
	    self.output.clear();
	} else if inputs[0] == "quit" {
	    self.output = "Quitting...".to_string();
	    self.closing = true;
	} else if matches!(inputs[0], "hello" | "hey" | "hi") {
	    self.output = "hi.".to_string();
	} else {
	    self.output = format!("I don't understand \"{}\"", self.input);
	}
    }
}

impl crate::app::Gui for Minibuffer {
    fn gui(&mut self, context: &egui::Context) {
	egui::TopBottomPanel::bottom("Minibuffer").resizable(false).show(context, |ui| {
	    ui.with_layout(
		vertical_layout(egui::Align::Min, true, egui::Direction::BottomUp), |ui| {
		    let minibuffer = ui.text_edit_singleline(&mut self.input);
		    if minibuffer.lost_focus() { //TODO: check for enter press instead of tab or outside click
			self.execute();
			self.input.clear();
		    }
		    minibuffer.request_focus();
		    ui.add_enabled(false, egui::Label::new(&self.output));
		},
	    );
	});
    }
}
