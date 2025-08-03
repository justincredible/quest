use eframe::egui;

#[derive(Default)]
pub struct Minibuffer {
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

    pub fn closing(&self) -> bool {
	self.closing
    }
}

impl crate::app::Gui for Minibuffer {
    fn gui(&mut self, context: &egui::Context) {
	egui::TopBottomPanel::bottom("Minibuffer").resizable(false).show(context, |ui| {
	    ui.with_layout(
		egui::Layout::bottom_up(egui::Align::Min)
		    .with_main_align(egui::Align::Max)
		    .with_cross_justify(true),
		|ui| {
		    let (enter_pressed, tab_pressed) = context.input(|i| {
			(i.key_pressed(egui::Key::Enter),
			 i.key_pressed(egui::Key::Tab))
		    });
		    let unfocused = context.memory(|m| m.focused().is_none());
		    let minibuffer = ui.text_edit_singleline(&mut self.input);
		    if minibuffer.lost_focus() && enter_pressed {
			self.execute();
			self.input.clear();
		    }
		    // refocus if enter pressed or outside sense
		    // set focus when nothing is (TODO: handle this more reliably)
		    if minibuffer.lost_focus() && !tab_pressed || unfocused {
			minibuffer.request_focus();
		    }
		    ui.add_enabled(false, egui::Label::new(&self.output));
		},
	    );
	});
    }
}
