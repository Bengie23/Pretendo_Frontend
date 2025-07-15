#![cfg_attr(
    all(
      target_os = "windows",
      not(debug_assertions),
    ),
    windows_subsystem = "windows"
  )]

mod connections;
mod logger;
mod data;
mod app;
mod associator;
mod pretendo_new_webhook_window;
mod pretendo_new_domain_window;
mod pretendo_header;
mod pretendo_domain_list;
mod pretendo_pretendos_list;
mod pretendo_loader;
mod backend_alive;
mod event_listener;

use app::MyApp;
use associator::Associator;
use eframe::egui::{self, Visuals};
use regex::Regex;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> eframe::Result {
    let image = image::load_from_memory(include_bytes!("../assets/pretendo-panda.png"))
            .unwrap()
            .to_rgba8();
    let (width, height) = image.dimensions();
    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default()
            .with_icon(std::sync::Arc::new(egui::IconData {
                rgba: image.to_vec(),
                width,
                height,
            }))
            .with_resizable(true)
            .with_maximize_button(false)
            .with_inner_size([1000.0, 700.0])
            .with_min_inner_size([840.0,700.0]),        
        ..Default::default()
    };
    let (tx_gui, rx_gui) = mpsc::unbounded_channel::<String>();
    let theme = Visuals::dark();
    eframe::run_native(
        "Pretendo App",
        options,
        Box::new(|cc| {
            let mut app = MyApp::new(rx_gui,tx_gui);
            app.populate_data();
            cc.egui_ctx.set_visuals(theme);
            cc.egui_ctx.set_pixels_per_point(1.5);
            
            Ok(Box::<MyApp>::new(app))
        }),
    )
}

pub fn validate_status_code(s: &mut String) {
    let re = Regex::new(r"[^0-9]+").unwrap();
    *s = re.replace_all(s, "").to_string();
    if s.len()>3{
        *s = s.chars().take(3).collect();
    }
}

pub fn sized_text(s: &str, fill: usize) -> String {
    let filling = " ".repeat(fill);
    return format!("{}{}", s, filling);
}