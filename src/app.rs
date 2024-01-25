use std::{sync::Arc, time::Duration};

use egui::{Color32, RichText, TopBottomPanel, Window};
use parking_lot::Mutex;

use crate::{args::RunArgs, misc::nullable::Nullable, nmea_0183::stores::Store};

pub struct App {
    pub args: RunArgs,
    pub store: Arc<Mutex<Store>>,
}

impl App {
    pub fn new(args: RunArgs, store: Arc<Mutex<Store>>) -> Self {
        Self { args, store }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(100));
        let store = self.store.lock();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Nautical Navigator");
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("[ {} ]", self.args.device));
                ui.label(format!("[ {:?} ]", store.location.fix));
            });
        });

        Window::new("Satellites").show(ctx, |ui| {
            let satellites = &store.satellites;
            ui.label(format!("Satellites in view: {}", satellites.in_view));
            ui.label(format!("Connected: {}", satellites.connected()));
            ui.separator();
            ui.label("Satellites:");

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
}
