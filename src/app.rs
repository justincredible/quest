use eframe::egui;
use crate::Gui;

pub enum App {
    Menu(crate::Menu),
}

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
	match self {
	    App::Menu(menu_type) => egui::CentralPanel::default().show(context, |ui| menu_type.gui(ui)),
	};
    }
}
