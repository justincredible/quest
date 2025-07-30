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

impl crate::app::Gui for Menu {
    fn gui(&mut self, ui: &mut egui::Ui) {
	match self {
	    Menu::Main => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
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
		    if ui.button("New").clicked() {
		    }
		    if ui.button("Load").clicked() {
		    }
		    if ui.button("Options").clicked() {
			*self = Menu::Options;
		    }
		    if ui.button("Help").clicked() {
			*self = Menu::Help;
		    }
		    if ui.button("Back").clicked() {
			*self = Menu::Main;
		    }
		});
	    },
	    Menu::Options => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    if ui.button("Quit").clicked() {
			ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
		    }
		    if ui.button("Back").clicked() {
			*self = Menu::Start;
		    }
		});
	    },
	    Menu::Help => {
		title_screen(ui);
		ui.vertical_centered(|ui| {
		    ui.label("God helps those who help themselves.");
		    ui.add_space(MENU_SPACE);
		    if ui.button("Help!").clicked() {
			todo!();
		    }
		    if ui.button("Back").clicked() {
			*self = Menu::Start;
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
	ui.add_space(2. * MENU_SPACE);
    });
}
