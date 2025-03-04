pub mod pretendo_ui{
    use eframe::egui::{self, SidePanel};
    use futures::executor::block_on;

    use crate::{app::MyApp, connections::http::PretendoHttpClient, data::entities::Pretendo};

    
    pub trait DomainsList {
        fn display_domains_list(&mut self, ctx: &egui::Context);    
    }

    impl DomainsList for MyApp {
        fn display_domains_list(&mut self, ctx: &egui::Context) {
            if self.backend_alive {
                if !self.display_new_domain{
                    let domains: Vec<String> = self.data.clone().into_iter().map(|element| element.domain).collect();
                    let lengths: Vec<i32> = domains.clone().into_iter().map(|element| element.len() as i32).collect();
                    let max = lengths.iter().max().unwrap_or(&1);
                    let default_width = max * 10;
                    SidePanel::left("left_panel").exact_width(default_width as f32).show(ctx, |ui| {
                        ui.heading("Domains");
                        let button = egui::Button::new("➕ Add domain");
                        let button_response = ui.add_sized(egui::vec2(ui.available_size().x, 20.0), button).on_hover_cursor(egui::CursorIcon::PointingHand);
                        if button_response.clicked() {
                            self.display_new_pretendo = false;
                            self.current_domain = String::from("");
                            self.display_new_domain = true;
                            self.current_pretendo = Pretendo::new();
                            self.pretendos_in_current_domain = Vec::new();
                        }
                        let _: Vec<_> = domains.iter().map(|domain| {
                            let label = egui::SelectableLabel::new(self.current_domain == *domain, domain);
                            let label_response = ui.add_sized(egui::vec2(ui.available_size().x, 20.0), label).on_hover_cursor(egui::CursorIcon::PointingHand);
                            if label_response.clicked() {
                                self.webhooks_in_current_pretendo = Vec::new();
                                self.current_webhook_payload = String::new();
                                self.current_webhook_url = String::new();
                                self.display_new_pretendo = false;
                                self.current_pretendo = Pretendo::new();
                                self.current_domain = domain.clone();
                                let json_response = block_on(PretendoHttpClient::get_pretendos(&domain));
                                if let Some(json_response) = json_response {
                                    let pretendos: Vec<Pretendo> = serde_json::from_str(&json_response).unwrap();
                                    self.pretendos_in_current_domain = pretendos;
                                }
                            }
                        }).collect();
                    });
                }
            }
        }
    }

}