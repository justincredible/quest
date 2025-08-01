use eframe::egui;

const TITLE_SIZE: f32 = 100.;
const MENU_SPACE: f32 = 25.;

pub enum Menu {
    Main,
    Start,
    Play(Option<usize>),
    Load,
    Options,
    Help,
}

impl crate::app::Gui for Menu {
    fn gui(&mut self, context: &egui::Context) {
	egui::CentralPanel::default().show(context, |ui| {
	    title_screen(ui);
	    match self {
		Menu::Main => {
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
		    ui.vertical_centered(|ui| {
			if ui.button("New").clicked() {
			    *self = Menu::Play(None);
			}
			if ui.button("Load").clicked() {
			    *self = Menu::Load;
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
		Menu::Play(_) => {
		    ui.vertical_centered(|ui| {
			ui.label("anderegwo...")
		    });
		},
		Menu::Load => {
		    ui.vertical_centered(|ui| {
			egui::ScrollArea::horizontal().show(ui, |ui| {
			    ui.label("Select game");
			});
			ui.add_space(MENU_SPACE);
			if ui.button("Back").clicked() {
			    *self = Menu::Start;
			}
		    });
		},
		Menu::Options => {
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
	});
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
