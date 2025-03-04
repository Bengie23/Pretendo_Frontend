pub mod pretendo_ui{
    use eframe::egui;

    use crate::{app::MyApp, data::entities::{Pretendo, PretendoElement}};

    
    pub trait NewDomainWindow {
        fn configure_new_domain_window(&mut self, ctx: &egui::Context);
    }

    impl NewDomainWindow for MyApp {
        fn configure_new_domain_window(&mut self, ctx: &egui::Context) {
            let mut new_domain_window = self.display_new_domain;
            if  new_domain_window {
                egui::Window::new("Add your custom domain")
                .resizable(false)
                .collapsible(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .open(&mut self.display_new_domain)
                    .show(ctx, |ui| {
                        let name_label = ui.label("Enter domain");
                        let text_response = ui.text_edit_singleline(&mut self.current_domain)
                        .labelled_by(name_label.id);
                        text_response.request_focus();
                        let enabled = !self.current_domain.is_empty();

                        let button_widget = egui::Button::new("Add domain");                    
                        
                        let button_response = ui.add_enabled(enabled, button_widget).on_hover_cursor(egui::CursorIcon::PointingHand);

                        if button_response.clicked() || text_response.lost_focus(){
                            if !self.current_domain.is_empty(){
                                self.data.push(PretendoElement {
                                    domain: self.current_domain.clone(),
                                    pretendos: [Pretendo::new()].to_vec(),
                                });
                                new_domain_window = false;
                            }
                        }
                    });
            }
            self.display_new_domain &= new_domain_window;
        }
    }

}