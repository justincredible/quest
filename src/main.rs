pub mod app;
use app::App;
pub mod menu;
use menu::Menu;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
	centered: true,
	..Default::default()
    };

    eframe::run_native("quest", options, Box::new(|_cc| Ok(Box::new(App::Menu(Menu::Main)))))
}
