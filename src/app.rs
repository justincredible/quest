use eframe::egui;

pub enum App {
    MainMenu,
}

const TITLE_SPACING: f32 = 25.;

impl eframe::App for App {
    fn update(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
	match *self {
	    App::MainMenu => {
		egui::CentralPanel::default().show(context, |ui| {
		    title_screen(ui);
		    ui.vertical_centered(|ui| {
			ui.add_space(TITLE_SPACING);
			if ui.button("Start").clicked() {
			}
		    });
		});
	    },
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
