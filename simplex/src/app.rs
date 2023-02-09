use crate::constraint::Constraints;
use crate::linear_function::LinearFunction;
use crate::Simplex;
use eframe::Frame;
use egui::{Color32, Context, Style};
use egui::TextStyle::Body;
use egui::TextStyle::Button;
use egui::TextStyle::Heading;
use egui::TextStyle::Monospace;
use egui::TextStyle::Small;
use egui::FontFamily::Proportional;
use egui::FontId;


#[derive(Debug)]
pub struct SimplexVisualizer {
    function_input: String,
    constraints_input: String,

    simplex: Option<Simplex>,
}
impl Default for SimplexVisualizer {
    fn default() -> Self {
        SimplexVisualizer {
            function_input: String::from("max x + 6y + 13z"),
            constraints_input: String::from("\
x <= 200\n\
y <= 300\n\
x + y + z <= 400\n\
y + 3z <= 600\n
            "),
            simplex: None
        }
    }
}
impl eframe::App for SimplexVisualizer {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		// Change font sizes
		let mut style = (*ctx.style()).clone();
		style.text_styles = [
			(Heading, FontId::new(30.0, Proportional)),
			(Body, FontId::new(24.0, Proportional)),
			(Monospace, FontId::new(14.0, Proportional)),
			(Button, FontId::new(14.0, Proportional)),
			(Small, FontId::new(10.0, Proportional)),
		]
		.into();
		ctx.set_style(style);

        egui::Area::new("Linear Program")
            .default_pos(egui::pos2(32f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Linear Program");
                            ui.text_edit_singleline(&mut self.function_input);
                            ui.text_edit_multiline(&mut self.constraints_input);

                            if ui.add(egui::Button::new("RUN")).clicked() {
                                // Parse constraints
                                let mut constraints = Constraints::default();
                                for line in self.constraints_input.lines().filter(|l| !l.trim().is_empty()) {
                                    constraints.add_constraint(
                                        line.parse().expect("invalid constraint input"),
                                    );
                                }

                                // Then create the resulting simplex instance
                                let (command, function) = {
                                    let mut words = self.function_input.split_ascii_whitespace();
                                    let command = words.next();
                                    let function_str =
                                        words.fold(String::new(), |acc, w| acc + w + " ");
                                    (
                                        command,
                                        function_str
                                            .parse::<LinearFunction>()
                                            .unwrap_or(LinearFunction::zero()),
                                    )
                                };

                                self.simplex = match command {
                                    Some("max") => Some(constraints.maximize(&function)),
                                    Some("min") => Some(constraints.minimize(&function)),
                                    _ => None,
                                };
                            }
                        });
                    })
            });

        egui::Area::new("State")
            .default_pos(egui::pos2(512f32, 512f32))
            .show(ctx, |ui| {
                egui::Frame::window(&Style::default())
                    .fill(Color32::BLACK)
                    .show(ui, |ui| {
                        ui.heading("State");
                        ui.vertical(|ui| {
                            if let Some(simplex) = &self.simplex {
                                let current_state = simplex.current_state();
                                ui.colored_label(
                                    Color32::LIGHT_GRAY,
                                    format!("{current_state}"),
                                );
                            } else {
                                ui.colored_label(
                                    Color32::LIGHT_GRAY,
                                    "Press RUN to start the algorithm",
                                );
                            }
                        });
                    })
            });

        // TODO: Step buttons

        // TODO: Figure display
    }
}
