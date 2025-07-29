use eframe::egui;

const TITLE_SPACING: f32 = 25.;

pub enum App {
    Menu(MenuType),
}

trait Gui {
    fn gui(&mut self, _ui: &mut egui::Ui);
}

pub enum MenuType {
    Main,
    Start,
}

impl Gui for MenuType {
    fn gui(&mut self, ui: &mut egui::Ui) {
	match self {
	    MenuType::Main => {
		ui.vertical_centered(|ui| {
		    ui.add_space(TITLE_SPACING);
		    if ui.button("Start").clicked() {
			*self = MenuType::Start;
		    }
		});
	    },
	    MenuType::Start => {
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
			*self = MenuType::Main;
		    }
		});
	    },
	    #[allow(unreachable_patterns)]
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
