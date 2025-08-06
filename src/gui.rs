pub trait Gui {
    fn gui(&mut self, _ctx: &eframe::egui::Context);
}
