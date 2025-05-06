use eframe::egui;
use egui::{Color32, Pos2, Stroke};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Real-Time Harmonagraph with Controls",
        options,
        Box::new(|_cc| Box::new(HarmonagraphApp::default())),
    )
}

struct HarmonagraphApp {
    t: f32,
    points: Vec<Pos2>,

    // Parameters with default values
    a1: f32,
    a2: f32,
    f1: f32,
    f2: f32,
    p1: f32,
    p2: f32,
    d1: f32,
    d2: f32,
}

impl Default for HarmonagraphApp {
    fn default() -> Self {
        Self {
            a1: 100.0,
            a2: 100.0,
            f1: 2.0,
            f2: 3.0,
            p1: 0.0,
            p2: std::f32::consts::PI / 2.0,
            d1: 0.002,
            d2: 0.003,
            t: 0.0,
            points: Vec::new()
        }
    }
}

impl eframe::App for HarmonagraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::SidePanel::left("controls_panel").show(ctx, |ui| {
            ui.heading("Harmonagraph Controls");

            ui.add(egui::Slider::new(&mut self.a1, 0.0..=200.0).text("Amplitude A1"));
            ui.add(egui::Slider::new(&mut self.a2, 0.0..=200.0).text("Amplitude A2"));

            ui.add(egui::Slider::new(&mut self.f1, 0.1..=10.0).text("Frequency f1"));
            ui.add(egui::Slider::new(&mut self.f2, 0.1..=10.0).text("Frequency f2"));

            ui.add(egui::Slider::new(&mut self.p1, 0.0..=std::f32::consts::TAU).text("Phase p1"));
            ui.add(egui::Slider::new(&mut self.p2, 0.0..=std::f32::consts::TAU).text("Phase p2"));

            ui.add(egui::Slider::new(&mut self.d1, 0.0..=0.01).text("Damping d1"));
            ui.add(egui::Slider::new(&mut self.d2, 0.0..=0.01).text("Damping d2"));

            if ui.button("🔁 Reset").clicked() {
                self.points.clear();
                self.t = 0.0;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let to_screen = |x: f32, y: f32| {
                let rect = response.rect;
                let center = rect.center();
                Pos2::new(center.x + x, center.y + y)
            };

            let dt = 0.01;
            let steps_per_frame = 3;

            for _ in 0..steps_per_frame {
                let t = self.t;
                let x = self.a1 * (self.f1 * t + self.p1).sin() * (-self.d1 * t).exp();
                let y = self.a2 * (self.f2 * t + self.p2).sin() * (-self.d2 * t).exp();

                let pos = to_screen(x, y);
                self.points.push(pos);
                self.t += dt;
            }

            for i in 1..self.points.len() {
                painter.line_segment(
                    [self.points[i - 1], self.points[i]],
                    Stroke::new(1.0, Color32::LIGHT_BLUE),
                );
            }
        });
    }
}
