use crate::evaluator::eval;
use crate::lexer::*;
use crate::parser::*;
use eframe::egui;

#[derive(Default)]
pub struct CalcApp {
    input: String,
    answer: f64,
    error_message: String,
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles.iter_mut().for_each(|(_, font_id)| {
            font_id.size = 24.0;
        });
        ctx.set_style(style);

        let btn = |label: &str| egui::Button::new(label);
        let btn_size = [100.0, 70.0];
        let display_size = [200.0, 50.0];
        let btn_cols = 4;
        let btn_width = btn_size[0] * btn_cols as f32;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_sized(display_size, egui::Label::new(&self.input));
                ui.add_sized(display_size, egui::Label::new(self.answer.to_string()));
                ui.add_sized(display_size, egui::Label::new(&self.error_message));

                let margin = (ui.available_width() - btn_width) / 2.0;

                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    if ui.add_sized(btn_size, btn("(")).clicked() {
                        self.input.push('(');
                    }
                    if ui.add_sized(btn_size, btn(")")).clicked() {
                        self.input.push(')');
                    }
                    if ui.add_sized(btn_size, btn("C")).clicked() {
                        self.input.clear();
                        self.answer = 0.0;
                        self.error_message.clear();
                    }
                    if ui.add_sized(btn_size, btn("<")).clicked() && !self.input.is_empty() {
                        self.input.pop();
                    };
                });
                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    for i in 7..=9 {
                        if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                            self.input.push_str(&i.to_string());
                        }
                    }
                    if ui.add_sized(btn_size, btn("/")).clicked() {
                        self.input.push('/');
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    for i in 4..=6 {
                        if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                            self.input.push_str(&i.to_string());
                        }
                    }
                    if ui.add_sized(btn_size, btn("*")).clicked() {
                        self.input.push('*');
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    for i in 1..=3 {
                        if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                            self.input.push_str(&i.to_string());
                        }
                    }
                    if ui.add_sized(btn_size, btn("-")).clicked() {
                        self.input.push('-');
                    }
                });
                ui.horizontal(|ui| {
                    ui.add_space(margin);
                    if ui.add_sized(btn_size, btn("0")).clicked() {
                        self.input.push('0');
                    }
                    if ui.add_sized(btn_size, btn(".")).clicked() {
                        self.input.push('.');
                    }
                    if ui.add_sized(btn_size, btn("=")).clicked() {
                        self.error_message.clear();
                        let mut lex = Lexer::new(&self.input);
                        if let Ok(tokens) = lex.tokenize() {
                            let mut parser = Parser::new(tokens);
                            if let Ok(expr) = parser.parse_expr() {
                                if let Ok(ev) = eval(&expr) {
                                    self.answer = ev;
                                } else {
                                    self.error_message = "err".to_string();
                                }
                            } else {
                                self.error_message = "err".to_string();
                            }
                        } else {
                            self.error_message = "err".to_string();
                        }
                    }
                    if ui.add_sized(btn_size, btn("+")).clicked() {
                        self.input.push('+');
                    }
                });
            });
        });
    }
}
