use eframe::egui;

const TITLE_SPACING: f32 = 25.;

pub enum Menu {
    Main,
    Start,
}

impl crate::Gui for Menu {
    fn gui(&mut self, ui: &mut egui::Ui) {
	match self {
	    Menu::Main => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    ui.add_space(TITLE_SPACING);
		    if ui.button("Start").clicked() {
			*self = Menu::Start;
		    }
		});
	    },
	    Menu::Start => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    ui.add_space(TITLE_SPACING);
		    if ui.button("New").clicked() {
		    }
		    if ui.button("Load").clicked() {
		    }
		    if ui.button("Options").clicked() {
		    }
		    if ui.button("Help").clicked() {
		    }
		    if ui.button("Back").clicked() {
			*self = Menu::Main;
		    }
		});
	    },
	    #[allow(unreachable_patterns)]
	    _ => ()
	}
    }
}

fn title_screen(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
	ui.add_space(TITLE_SPACING);
	ui.label(egui::RichText::new("This is")
		 .heading()
		 .strikethrough());
	ui.heading("Welcome to");
	ui.add_space(TITLE_SPACING);
	ui.colored_label(
	    egui::Color32::PURPLE,
	    egui::RichText::new("QUEST")
		.heading()
		.monospace()
		.strong()
		.underline()
		.size(crate::TITLE_SIZE));
	ui.add_space(TITLE_SPACING);
    });
}
