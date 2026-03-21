use eframe::egui;
use std::time::{Duration, Instant};

struct StopwatchApp {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Default for StopwatchApp {
    fn default() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
            running: false,
        }
    }
}

impl eframe::App for StopwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Update elapsed time if running
            if self.running {
                if let Some(start) = self.start_time {
                    self.elapsed = start.elapsed();
                }
            }

            // Display time
            let secs = self.elapsed.as_secs() % 60;
            let mins = (self.elapsed.as_secs() / 60) % 60;
            let hours = self.elapsed.as_secs() / 3600;
            ui.label(format!("{:02}:{:02}:{:02}", hours, mins, secs));

            ui.horizontal(|ui| {
                if ui.button("Start").clicked() {
                    if !self.running {
                        self.start_time = Some(Instant::now() - self.elapsed);
                        self.running = true;
                    }
                }

                if ui.button("Stop").clicked() {
                    if self.running {
                        if let Some(start) = self.start_time {
                            self.elapsed = start.elapsed();
                        }
                        self.running = false;
                    }
                }

                if ui.button("Reset").clicked() {
                    self.elapsed = Duration::ZERO;
                    self.start_time = if self.running { Some(Instant::now()) } else { None };
                }
            });
        });

        // Make egui repaint continuously so the timer updates
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Stopwatch",
        options,
        Box::new(|_cc| Ok(Box::new(StopwatchApp::default()) as Box<dyn eframe::App>)),
    )
}