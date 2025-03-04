pub mod pretendo_ui{
    use eframe::egui::{self, Button, CentralPanel, TextEdit, TopBottomPanel, Ui};
    use futures::executor::block_on;

    use crate::{app::MyApp, connections::{self, http::PretendoHttpClient}, data::entities::{Pretendo, Webhook}, logger, sized_text, validate_status_code};

    
    pub trait PretendosList {
        fn display_pretendos_list(&mut self, ctx: &egui::Context);    
    }

    impl PretendosList for MyApp{
        fn display_pretendos_list(&mut self, ctx: &egui::Context) {
            if self.backend_alive {
                let current_pretendo_name = self.current_pretendo.name.clone();

                if !self.display_new_domain && !self.current_domain.is_empty(){
                    TopBottomPanel::top("top_panel").show(ctx, |ui| {
                        ui.heading("Pretendos");
                        ui.horizontal(|ui| {
            
                            for item in self.pretendos_in_current_domain.clone() {
                                let pretendo = item.clone();
                                let selectable_label = ui.selectable_label(item.name == current_pretendo_name, item.name).on_hover_cursor(egui::CursorIcon::PointingHand);
                                if selectable_label.clicked(){
                                    self.display_new_pretendo = true;
                                    self.current_pretendo = pretendo;
                                    self.webhooks_in_current_pretendo = Vec::new();
                                    self.current_webhook_is_get =false;
                                    self.current_webhook_is_post = false;
                                    self.current_webhook_http_verb = None;
                                    self.current_webhook_payload = String::new();
                                    self.current_webhook_url = String::new();
                                    self.current_webhook_delay = 0;

                                }
                            }
                            let button = ui.button("➕ New Pretendo").on_hover_cursor(egui::CursorIcon::PointingHand);
                            if button.clicked() {
                                self.display_new_pretendo = true;
                                self.current_pretendo = Pretendo::new();
                                self.current_webhook_is_get =false;
                                self.current_webhook_is_post = false;
                                self.current_webhook_http_verb = None;
                                self.current_webhook_payload = String::new();
                                self.current_webhook_url = String::new();
                                self.current_webhook_delay = 0;
                            }
            
                        });
                    });
                }
                if self.display_new_pretendo {     
                    CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical().animated(true).max_height(ui.available_height()).show(ui, |ui| {
                            let margin_right = 0.0;
                            let domain_copy = self.current_domain.clone();
                            let mut url = String::from("Url: 'http://'");
                            if !self.current_domain.is_empty() {
                                url = format!("http://{}{}", domain_copy, self.current_pretendo.path);
                            }
                            
                            let tooltip_text = "Click to open in browser";
                                ui.hyperlink(url).on_hover_text(tooltip_text);

                            ui.horizontal(|ui| {                    
                                ui.add(egui::Label::new(sized_text("Domain:", 13)));
                                ui.add_enabled_ui(false, |ui|{
                                    ui.add_sized(egui::vec2(ui.available_size().x - margin_right,20.0), egui::TextEdit::singleline(&mut self.current_domain.clone()));
                                });
                                
                            });
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(sized_text("Name:", 17)));
                                ui.add_sized(egui::vec2(ui.available_size().x - margin_right, 20.0), egui::TextEdit::singleline(&mut self.current_pretendo.name));
                            });
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(sized_text("Path:", 20)));
                                ui.add_sized(egui::vec2(ui.available_size().x - margin_right, 20.0), egui::TextEdit::singleline(&mut self.current_pretendo.path));
                            });
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(sized_text("Return Object:", 1)));
                                
                                let win_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
                                ui.add_sized(
                                    egui::vec2(ui.available_size().x - margin_right, ui.available_size().y + (win_rect.height() - 400.0)), 
                                    TextEdit::multiline(&mut self.current_pretendo.return_object).code_editor().lock_focus(false));
                                
                            });
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Label::new(sized_text("Status Code:", 5)));
                                let text_response = ui.add_sized(egui::vec2(30.0 ,20.0), egui::TextEdit::singleline(&mut self.current_pretendo.status_code));
                                if text_response.changed() {
                                    validate_status_code(&mut self.current_pretendo.status_code);
                                }
                            });
                            ui.add_space(15.0);
                            ui.horizontal(|ui:&mut Ui| {
                                ui.add_sized(egui::vec2(220.0, 30.0), |ui: &mut Ui| {
                                    let element = Button::new("💾 Save pretendo");
                                    let output = ui.add_enabled(self.current_pretendo.id.is_none(), element).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    if output.clicked() {
                                        let replaced_return_object = self.current_pretendo.return_object.clone().replace("\t", "").replace("\n", "").replace("\"", "'");
                                        let pretendo_creation = 
                                            PretendoHttpClient::add_pretendo(
                                                &self.current_domain, 
                                                &self.current_pretendo.path, 
                                                &replaced_return_object,
                                                &self.current_pretendo.name,
                                                &self.current_pretendo.status_code);
                                        let pretendo_creation_attempt = block_on(pretendo_creation);
                                        if pretendo_creation_attempt.is_ok(){
                                            self.display_new_domain = false;
                                            self.display_new_pretendo = false;
                                            self.current_pretendo = Pretendo::new();
                                            let json_response = block_on(PretendoHttpClient::get_pretendos(&self.current_domain));
                                            if let Some(json_response) = json_response {
                                                let pretendos: Vec<Pretendo> = serde_json::from_str(&json_response).unwrap();
                                                self.pretendos_in_current_domain = pretendos.clone();
                                                let just_created = self.pretendos_in_current_domain.last();
                                                if just_created.is_some(){
                                                    self.current_pretendo = just_created.unwrap().clone();
                                                    self.display_new_pretendo = true;
                                                }
                                            }
                                        }
                                    }
                                    output
                                });
                                ui.add_sized(egui::vec2(220.0, 30.0), |ui: &mut Ui| {
                                    let element = Button::new("Configure Webhook(s)");
                                    let output = ui.add_enabled(self.current_pretendo.id.is_some(), element).on_hover_cursor(egui::CursorIcon::PointingHand);
                                    if output.clicked() {
                                        logger::debug::println!("Configuring webhooks");
                                        let json_response = block_on(connections::http::PretendoHttpClient::get_webhooks(&self.current_pretendo.id.unwrap()));
                                        logger::debug::println!("{:?}", json_response);
                                        if let Some(json_response) = json_response {
                                            let webhooks: Vec<Webhook> = serde_json::from_str(&json_response).unwrap();
                                            logger::debug::println!("webhooks for current pretendo {:?}",webhooks);
                                            self.webhooks_in_current_pretendo = webhooks;
                                            if self.webhooks_in_current_pretendo.len() > 0 {
                                                let current_webhook = self.webhooks_in_current_pretendo.first().cloned().unwrap();
                                                self.current_webhook_payload = current_webhook.payload;
                                                self.current_webhook_url = current_webhook.url;
                                                self.current_webhook_http_verb = Some(current_webhook.http_verb);
                                                self.current_webhook_delay = current_webhook.delay;
                                            }
                                            else{
                                                self.current_webhook_payload = String::new();
                                                self.current_webhook_url = String::new();
                                                self.current_webhook_http_verb = None;
                                                self.current_webhook_delay = 0;
                                            }
                                        }
                                        self.display_configure_webhooks = true;
                                    }
                                    output
                                });
                            });
                        });
                    });
                }
            }
        }
    }

}