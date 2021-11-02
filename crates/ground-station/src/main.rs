use eframe::{
    egui,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");

            if ui.button("pee pee").clicked() {
                frame.quit()
            }
        });
    }
}

fn create_icon() -> IconData {
    const SIZE: i32 = 1028;

    fn f64_to_u8(f: f64) -> u8 {
        (f * 255.0).clamp(0.0, 255.0).round() as u8
    }

    let rgba = (0..1028 * 1028)
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
        height: 1028,
        width: 1028,
        rgba: rgba.collect(),
    }
}

fn main() {
    eframe::run_native(
        Box::<GroundStation>::default(),
        NativeOptions {
            icon_data: Some(create_icon()),
            maximized: true,
            decorated: false,
            ..Default::default()
        },
    );
}
