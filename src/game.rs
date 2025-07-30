use eframe::egui;

pub enum Game {
    New,
    Loaded,
    Exited,
}

impl crate::app::Gui for Game {
    fn gui(&mut self, ui: &mut egui::Ui) {
	ui.label("oops...");
	todo!();
    }
}
