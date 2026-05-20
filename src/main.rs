mod evaluator;
mod lexer;
mod parser;
mod ui;

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(ui::CalcApp::default()))),
    )
    .unwrap();
}
