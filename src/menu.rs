use eframe::egui;

const TITLE_SIZE: f32 = 100.;
const MENU_SPACE: f32 = 25.;

pub enum Menu {
    Main,
    Start,
    Load,
    Options,
    Help,
}

impl crate::Gui for Menu {
    fn gui(&mut self, ui: &mut egui::Ui) {
	match self {
	    Menu::Main => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    ui.add_space(MENU_SPACE);
		    if ui.button("Start").clicked() {
			*self = Menu::Start;
		    }
		    if ui.button("Quit").clicked() {
			ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
		    }
		});
	    },
	    Menu::Start => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    ui.add_space(MENU_SPACE);
		    if ui.button("New").clicked() {
		    }
		    if ui.button("Load").clicked() {
		    }
		    if ui.button("Options").clicked() {
		    }
		    if ui.button("Help").clicked() {
			*self = Menu::Help;
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
	ui.add_space(MENU_SPACE);
	ui.label(
	    egui::RichText::new("This is")
		.heading()
		.strikethrough());
	ui.heading("Welcome to");
	ui.add_space(MENU_SPACE);
	ui.colored_label(
	    egui::Color32::PURPLE,
	    egui::RichText::new("QUEST")
		.heading()
		.monospace()
		.strong()
		.underline()
		.size(TITLE_SIZE));
	ui.add_space(MENU_SPACE);
    });
}
