use std::{sync::Arc, time::Duration};

use egui::{Align, Color32, Layout, RichText, ScrollArea, SidePanel, TopBottomPanel, Window};
use egui_plot::{Line, Plot};
use parking_lot::Mutex;

use crate::{
    args::RunArgs, consts::HISTORY_SAMPLES, log::Log, misc::nullable::Nullable,
    nmea_0183::stores::Store,
};

pub struct App {
    pub args: RunArgs,
    pub log: Log,
    pub store: Arc<Mutex<Store>>,

    show_windows: bool,
    show_log: bool,
    show_satellites: bool,
    show_location: bool,
}

impl App {
    pub fn new(args: RunArgs, store: Arc<Mutex<Store>>, log: Log) -> Self {
        Self {
            args,
            store,
            log,

            show_windows: false,
            show_log: true,
            show_satellites: true,
            show_location: true,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));
        let store = self.store.lock();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nautical Navigator");
                ui.separator();
                ui.toggle_value(&mut self.show_windows, "🗖 Windows");
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("[ {} ]", self.args.device));
                ui.label(format!("[ {:?} ]", store.location.fix));
            });
        });

        if self.show_windows {
            SidePanel::right("windows_panel")
                .default_width(100.0)
                .show(ctx, |ui| {
                    ui.heading("Windows");
                    ui.separator();
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        ui.toggle_value(&mut self.show_log, "📜 Log");
                        ui.toggle_value(&mut self.show_satellites, "🚀 Satellites");
                        ui.toggle_value(&mut self.show_location, "📍 Position");
                    });
                });
        }

        if self.show_satellites {
            Window::new("Satellites").show(ctx, |ui| {
                let satellites = &store.satellites;

                ui.heading("Overview");
                ui.label(format!("Satellites in view: {}", satellites.in_view));
                ui.label(format!("Connected: {}", satellites.connected()));

                ui.add_space(12.0);
                ui.heading("SNR History");
                let line = Line::new(
                    store
                        .satellites
                        .avg_sdr_history
                        .iter()
                        .enumerate()
                        .map(|x| [x.0 as f64 - HISTORY_SAMPLES as f64, *x.1 as f64])
                        .collect::<Vec<_>>(),
                );
                Plot::new("snr_history")
                    .allow_drag(false)
                    .view_aspect(2.0)
                    .show(ui, |plot_ui| plot_ui.line(line));

                ui.add_space(12.0);
                ui.heading("Satellites");
                for (i, satellite) in satellites.satellites.iter().enumerate() {
                    let color = if satellite.snr.is_some() {
                        Color32::PLACEHOLDER
                    } else {
                        Color32::DARK_GRAY
                    };

                    ui.push_id(i, |ui| {
                        ui.collapsing(
                            RichText::new(format!("Satellite {}", satellite.id)).color(color),
                            |ui| {
                                ui.label(format!("ID: {}", satellite.id));
                                ui.label(format!("Elevation: {}", Nullable(satellite.elevation)));
                                ui.label(format!("Azimuth: {}", Nullable(satellite.azimuth)));
                                ui.label(format!("SNR: {}", Nullable(satellite.snr)));
                            },
                        )
                    });
                }
            });
        }

        if self.show_location {
            Window::new("Position").show(ctx, |ui| {
                let location = &store.location;
                ui.label(format!("Latitude: {:?}", location.latitude));
                ui.label(format!("Longitude: {:?}", location.longitude));
                ui.label(format!("Time: {:?}", location.time));
                ui.label(format!("Status: {:?}", location.status));
                ui.label(format!("Fix: {:?}", location.fix));
                ui.label(format!("PDOP: {}", location.pdop));
                ui.label(format!("HDOP: {}", location.hdop));
                ui.label(format!("VDOP: {}", location.vdop));
            });
        }

        if self.show_log {
            Window::new("Log").default_width(800.0).show(ctx, |ui| {
                let entries = self.log.entries();

                ScrollArea::vertical().show(ui, |ui| {
                    for (i, entry) in entries.iter().enumerate().rev() {
                        let color = match entry.level {
                            crate::log::LogLevel::Info => Color32::GRAY,
                            crate::log::LogLevel::Warning => Color32::YELLOW,
                            crate::log::LogLevel::Error => Color32::RED,
                        };

                        ui.push_id(i, |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "[{}] {}",
                                    entry.timestamp.format("%H:%M:%S"),
                                    entry.message
                                ))
                                .color(color),
                            );
                        });
                    }
                });
            });
        }
    }
}
