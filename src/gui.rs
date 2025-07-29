use eframe::egui;

pub trait Gui {
    fn gui(&mut self, _ui: &mut egui::Ui);
}
