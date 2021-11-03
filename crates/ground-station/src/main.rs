use eframe::{
    egui::{self, Grid, Label},
    epi::{self, IconData},
    NativeOptions,
};

#[derive(Default)]
struct GroundStation {}

impl epi::App for GroundStation {
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let window_size = ctx.available_rect();
        let width = window_size.width() / 6.0;
        let height = window_size.height() / 6.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.add_sized((width, height), Label::new("a"));
                ui.add_sized((width, height), Label::new("b"));
                ui.add_sized((width, height), Label::new("c"));
                ui.add_sized((width, height), Label::new("d"));
                ui.add_sized((width, height), Label::new("e"));
                ui.add_sized((width, height), Label::new("f"));
            });

            ui.vertical(|ui| {
                ui.add_sized((width, height), Label::new("b"));
                ui.add_sized((width, height), Label::new("c"));
                ui.add_sized((width, height), Label::new("d"));
                ui.add_sized((width, height), Label::new("e"));
                ui.add_sized((width, height), Label::new("f"));
            });

            ui.vertical(|ui| {
                ui.add_sized((width, height), Label::new("b"));
                ui.add_sized((width, height), Label::new("c"));
                ui.add_sized((width, height), Label::new("d"));
                ui.add_sized((width, height), Label::new("e"));
                ui.add_sized((width, height), Label::new("f"));
            });
            Grid::new("layout_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("First row, first column");
                    ui.label("First row, second column");
                    ui.end_row();

                    ui.label("Second row, first column");
                    ui.label("Second row, second column");
                    ui.label("Second row, third column");
                    ui.end_row();

                    ui.horizontal(|ui| {
                        ui.label("Same");
                        ui.label("cell");
                    });
                    ui.label("Third row, second column");
                    if ui.button("pee pee").clicked() {
                        frame.quit()
                    }
                    ui.end_row();
                });
        });
    }
}

fn create_icon() -> IconData {
    const SIZE: i32 = 256;

    fn f64_to_u8(f: f64) -> u8 {
        (f * 255.0).clamp(0.0, 255.0).round() as u8
    }

    let rgba = (0..SIZE * SIZE)
        .map(|i| (i % SIZE - SIZE / 2, i / SIZE - SIZE / 2))
        .map(|(x, y)| (x as f64 / (SIZE / 2) as f64, y as f64 / (SIZE / 2) as f64))
        .map(|(x, y)| {
            [
                f64_to_u8(x.abs()),
                f64_to_u8(0.5 - (x * x + y * y)),
                f64_to_u8(y.abs()),
                if x * x + y * y > 1.0 { 0 } else { u8::MAX },
            ]
        })
        .flatten();

    IconData {
        height: SIZE as u32,
        width: SIZE as u32,
        rgba: rgba.collect(),
    }
}

fn main() {
    eframe::run_native(
        Box::<GroundStation>::default(),
        NativeOptions {
            icon_data: Some(create_icon()),
            // maximized: true,
            // decorated: false,
            ..Default::default()
        },
    );
}
