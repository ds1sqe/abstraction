use eframe::{
    egui::{self, Color32, Stroke},
    emath::Numeric,
};
use egui_plot::{
    HLine, Legend, Line, LineStyle, PlotItem, PlotPoint, PlotPoints, Points, Polygon, VLine,
};

use crate::core::model::{Limit, Model, SingleConstrain};

pub struct Manager {
    model: Model,
}

impl Default for Manager {
    fn default() -> Self {
        let mut model = Model::new();
        model.add_fixed(0, 10);
        model.add_boundary(
            0,
            Some(Limit {
                point: 20,
                equal: false,
            }),
            Some(Limit {
                point: 0,
                equal: true,
            }),
        );

        Self { model }
    }
}

impl Manager {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for Manager {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            egui_plot::Plot::new("plot..")
                .width(600.0)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    // plot_ui.polygon(
                    //     Polygon::new(vec![[0.0, 0.0], [5.0, 5.0], [5.0, 1.0]])
                    //         .style(egui_plot::LineStyle::Dashed { length: 12.0 })
                    //         .stroke(Stroke::new(3.0, Color32::from_rgb(100, 100, 200)))
                    //         .fill_color(Color32::from_rgb(200, 200, 200)),
                    // );

                    for (id, sc_vec) in self.model.single.iter() {
                        for sc in sc_vec {
                            match sc {
                                SingleConstrain::Fixed(f) => plot_ui.hline(
                                    HLine::new(f.value.to_f64())
                                        .name(format!("Fixed : {:?}", f))
                                        .width(3.0),
                                ),
                                SingleConstrain::Boundary(b) => {
                                    if let Some(bot) = b.bot {
                                        plot_ui.hline(
                                            HLine::new(bot.limit.point.to_f64())
                                                .style(match bot.limit.equal {
                                                    true => LineStyle::Dotted { spacing: 10.0 },
                                                    false => LineStyle::Solid,
                                                })
                                                .name(format!("Bottom : {:?}", bot.limit.point))
                                                .width(3.0),
                                        );
                                    }
                                    if let Some(top) = b.top {
                                        plot_ui.hline(
                                            HLine::new(top.limit.point.to_f64())
                                                .style(match top.limit.equal {
                                                    true => LineStyle::Dotted { spacing: 10.0 },
                                                    false => LineStyle::Solid,
                                                })
                                                .name(format!("Top : {:?}", top.limit.point))
                                                .width(3.0),
                                        );
                                    }
                                }
                            }
                        }
                    }
                });

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        egui::SidePanel::right("side bar").show(ctx, |ui| {
            ui.separator();
            ui.menu_button("add constraint", |ui| {
                if ui.button("Boundary").clicked() {}
                if ui.button("Fixed").clicked() {}
                if ui.button("Relation").clicked() {}
            });

            if ui.button("Open Plot").clicked() {}
        });
    }
}
