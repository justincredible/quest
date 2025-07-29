pub mod app;
use app::{App, MenuType};

const TITLE_SIZE: f32 = 100.;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
	centered: true,
	..Default::default()
    };

    eframe::run_native("quest", options, Box::new(|_cc| Ok(Box::new(App::Menu(MenuType::Main)))))
}
