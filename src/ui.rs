use crate::evaluator::eval;
use crate::lexer::*;
use crate::parser::*;
use eframe::egui;

const BTN_ROWS: f32 = 5.0;
const BTN_COLS: usize = 5;
const LABEL_ROWS: f32 = 3.0;
const LABEL_H: f32 = 50.0;
const MEMORY_SLOTS: usize = 5;

/// 空白付き演算子のリスト。バックスペースでまとめて削除するために使用
const OPS: &[&str] = &[" + ", " - ", " * ", " / "];

/// 電卓アプリの状態
pub struct CalcApp {
    /// 入力中の式
    input: String,
    /// 直前の計算結果
    answer: f64,
    /// エラーメッセージ（正常時は空）
    error_message: String,
    /// メモリスロット（M1〜M5）
    memory: [f64; MEMORY_SLOTS],
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            answer: 0.0,
            error_message: String::new(),
            memory: [0.0; MEMORY_SLOTS],
        }
    }
}

impl CalcApp {
    /// 末尾が空白付き演算子なら3文字まとめて削除、それ以外は1文字削除
    fn backspace(&mut self) {
        for op in OPS {
            if self.input.ends_with(op) {
                let new_len = self.input.len() - op.len();
                self.input.truncate(new_len);
                return;
            }
        }
        self.input.pop();
    }

    /// input をパイプライン（Lexer→Parser→Evaluator）で評価して answer を更新
    fn calculate(&mut self) {
        self.error_message.clear();
        // 未閉じ括弧を自動補完
        let open = self.input.chars().filter(|&c| c == '(').count();
        let close = self.input.chars().filter(|&c| c == ')').count();
        for _ in 0..(open.saturating_sub(close)) {
            self.input.push(')');
        }
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
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles.iter_mut().for_each(|(_, font_id)| {
            font_id.size = 24.0;
        });
        ctx.set_style(style);

        let btn = |label: &str| egui::Button::new(label);

        egui::SidePanel::right("memory_panel")
            .resizable(false)
            .exact_width(140.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label("Memory");
                    ui.separator();
                    for i in 0..MEMORY_SLOTS {
                        ui.horizontal(|ui| {
                            ui.label(format!("M{}: {}", i + 1, self.memory[i]));
                        });
                        ui.horizontal(|ui| {
                            if ui.button("S").clicked() {
                                self.memory[i] = self.answer;
                            }
                            if ui.button("R").clicked() {
                                self.input.push_str(&self.memory[i].to_string());
                            }
                        });
                        ui.separator();
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style()).inner_margin(egui::Margin {
                    left: 0,
                    right: 16,
                    top: 0,
                    bottom: 25,
                }),
            )
            .show(ctx, |ui| {
                let w = ui.available_width();
                let h = ui.available_height();
                let label_total = LABEL_H * LABEL_ROWS;
                let btn_h = ((h - label_total) / BTN_ROWS).max(30.0);
                let btn_w = w / BTN_COLS as f32;
                let btn_size = [btn_w, btn_h];
                let display_size = [w, LABEL_H];

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_sized(display_size, egui::Label::new(&self.input));
                    ui.add_sized(display_size, egui::Label::new(self.answer.to_string()));
                    ui.add_sized(display_size, egui::Label::new(&self.error_message));

                    ui.horizontal(|ui| {
                        // 左列: 空白
                        ui.add_sized(btn_size, egui::Label::new(""));
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
                        if ui.add_sized(btn_size, btn("<")).clicked() {
                            self.backspace();
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.add_sized(btn_size, btn("y\u{221a}x")).clicked() {
                            let x = self.input.clone();
                            self.input = format!("sqrt({},", x);
                        }
                        for i in 7..=9 {
                            if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                                self.input.push_str(&i.to_string());
                            }
                        }
                        if ui.add_sized(btn_size, btn("/")).clicked() {
                            self.input.push_str(" / ");
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.add_sized(btn_size, btn("x^y")).clicked() {
                            let x = self.input.clone();
                            self.input = format!("pow({},", x);
                        }
                        for i in 4..=6 {
                            if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                                self.input.push_str(&i.to_string());
                            }
                        }
                        if ui.add_sized(btn_size, btn("*")).clicked() {
                            self.input.push_str(" * ");
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.add_sized(btn_size, btn("log")).clicked() {
                            let x = self.input.clone();
                            self.input = format!("log({},", x);
                        }
                        for i in 1..=3 {
                            if ui.add_sized(btn_size, btn(&i.to_string())).clicked() {
                                self.input.push_str(&i.to_string());
                            }
                        }
                        if ui.add_sized(btn_size, btn("-")).clicked() {
                            self.input.push_str(" - ");
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui.add_sized(btn_size, btn("+/-")).clicked() { /* TODO */ }
                        if ui.add_sized(btn_size, btn("0")).clicked() {
                            self.input.push('0');
                        }
                        if ui.add_sized(btn_size, btn(".")).clicked() {
                            self.input.push('.');
                        }
                        if ui.add_sized(btn_size, btn("=")).clicked() {
                            self.calculate();
                        }
                        if ui.add_sized(btn_size, btn("+")).clicked() {
                            self.input.push_str(" + ");
                        }
                    });
                });
            });
    }
}
