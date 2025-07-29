use eframe::egui;

const TITLE_SPACING: f32 = 25.;

pub enum App {
    Menu(MenuType),
}

trait Gui {
    fn gui(&self, _ui: &mut egui::Ui);
}

pub enum MenuType {
    Main,
    Start,
}

impl Gui for MenuType {
    fn gui(&self, ui: &mut egui::Ui) {
	match self {
	    MenuType::Main => {
		ui.vertical_centered(|ui| {
		    ui.add_space(TITLE_SPACING);
		    if ui.button("Start").clicked() {
		    }
		});
	    },
	    _ => ()
	}
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &eframe::egui::Context, _frame: &mut eframe::Frame) {
	match self {
	    App::Menu(menu_type) => {
		egui::CentralPanel::default().show(context, |ui| {
		    title_screen(ui);
		    menu_type.gui(ui);
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
