use egui::{Align, Layout, Pos2, Vec2};
use egui_extras::{Size, StripBuilder};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 853;
const WINDOW_HEIGHT: i32 = 480;

fn conf() -> Conf {
    Conf {
        window_title: String::from("egui in macroquad"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let (mut w, mut h) = (0.0, 0.0);
    loop {
        clear_background(WHITE);
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("The Tower")
                .default_pos(Pos2::new(
                    (WINDOW_WIDTH / 2) as f32 - 200.0,
                    (WINDOW_HEIGHT / 2) as f32 - 80.0,
                ))
                .default_size(Vec2::new(400.0, 320.0))
                .show(egui_ctx, |ui| {
                    ui.add_space(20.0);
                    ui.vertical_centered(|ui| {
                        ui.label("What's the right thing to do?");
                    });
                    ui.add_space(32.0);
                    StripBuilder::new(ui)
                        .size(Size::exact(32.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                StripBuilder::new(ui)
                                    .size(Size::remainder()) // for the table
                                    .size(Size::exact(120.0)) // for the table
                                    .size(Size::exact(120.0)) // for the source code link
                                    .size(Size::remainder()) // for the table
                                    .horizontal(|mut strip| {
                                        strip.empty();

                                        strip.cell(|ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.button("Return the sun");
                                            });
                                        });
                                        strip.cell(|ui| {
                                            ui.centered_and_justified(|ui| {
                                                ui.button("Return home");
                                            });
                                        });
                                        strip.empty();
                                    });
                            });
                        });
                    ui.add_space(16.0);
                });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
