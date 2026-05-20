mod evaluator;
mod lexer;
mod parser;
mod ui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(ui::CalcApp::default()))),
    )
    .unwrap();
}
