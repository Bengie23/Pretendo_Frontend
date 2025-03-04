pub mod pretendo_ui{
    use eframe::egui::{self, TopBottomPanel};

    use crate::{app::MyApp, backend_alive::utils::BackendAlive};

    pub trait PretendoLoader {
        fn display_loader(&mut self, ctx: &egui::Context);
    }
    
    impl PretendoLoader for MyApp {
        fn display_loader(&mut self, ctx: &egui::Context) {
            TopBottomPanel::top("panel_loader").show_animated(ctx,!self.backend_alive, |ui| {
                ui.label(r#"Backend not runnning....
                - Pretendo Backend must be running for the PRETENDO UI to work."#);
                ui.add_space(15.0);
                ui.vertical_centered( |ui|{
                    
                    let button_widget = egui::Button::new("  Reload  ↻  ");                    
                    
                    let button_response = ui.add_visible(!self.backend_alive, button_widget).on_hover_cursor(egui::CursorIcon::PointingHand);
                    
                    ui.add_visible(self.running_backend_ping_pong, egui::widgets::Spinner::new());
                    if button_response.clicked() {
                        self.check_backend(ctx);
                    }
                });
            });
        }
    }
}