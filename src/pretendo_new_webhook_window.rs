pub mod pretendo_ui{
    use eframe::egui::{self, Ui};
    use futures::executor::block_on;

    use crate::{app::MyApp, connections, data::entities::HttpVerbs, logger, sized_text};

    pub trait NewWebhookWindow{
        fn configure_new_webhook_window(&mut self, ctx: &egui::Context);
    }

    impl NewWebhookWindow for MyApp {
        fn configure_new_webhook_window(&mut self, ctx: &egui::Context) {
            if self.backend_alive {
                let mut should_display = self.display_configure_webhooks;
                if self.current_webhook_http_verb.is_some(){
                    let current_value = self.current_webhook_http_verb.clone().unwrap();
                    if current_value == HttpVerbs::GET{
                        self.current_webhook_is_get = true;
                    }
                    if current_value == HttpVerbs::POST{
                        self.current_webhook_is_post = true;
                    }
                }
                if  should_display {
                    let webhook_window = egui::Window::new("Configure Webhooks")
                    .collapsible(false)
                    .open(&mut self.display_configure_webhooks)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui::Label::new(sized_text("Url:", 11)));
                            let _text_response = ui.add_sized(egui::vec2(ui.available_size().x ,20.0), egui::TextEdit::singleline(&mut self.current_webhook_url));
                        });

                        ui.add_space(2.5);
                        ui.horizontal(|ui| {
                        ui.add(egui::Label::new(sized_text("Verb:", 8)));

                            let get_option = ui.checkbox(&mut self.current_webhook_is_get, "HTTP GET");
                            if get_option.clicked(){
                                if self.current_webhook_is_get
                                {
                                    self.current_webhook_http_verb = Some(HttpVerbs::GET); 
                                    self.current_webhook_is_post = false;   
                                }
                            }
                            let post_option = ui.checkbox(&mut self.current_webhook_is_post, "HTTP POST");
                            if post_option.clicked(){
                                if self.current_webhook_is_post
                                {
                                    self.current_webhook_http_verb = Some(HttpVerbs::POST);
                                    self.current_webhook_is_get = false; 
                                }
                            }
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            let widget = egui::TextEdit::multiline(&mut self.current_webhook_payload)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(false)
                            .desired_width(f32::INFINITY);

                            ui.add(egui::Label::new(sized_text("Payload:", 1)));
                            
                            egui::ScrollArea::vertical().min_scrolled_height(300.0).show(ui, |ui| {

                                ui.add_sized(egui::vec2(ui.available_size().x, 300.0), |ui: &mut Ui|{
                                    let enabled_widget = ui.add_enabled(  self.current_webhook_is_post, widget);
                                    
                                    return enabled_widget;
                                });
                            });
                            
                        });
                        ui.add_space(2.5);
                        ui.horizontal(|ui|{
                            ui.add(egui::Label::new(sized_text("Delay:", 7)));
                            ui.add(egui::Slider::new( &mut self.current_webhook_delay, 0..=60).suffix(" secs"));
                        });
                        
                        ui.add_space(2.5);
                        let enabled = self.webhooks_in_current_pretendo.len() == 0 && !self.current_webhook_url.is_empty() && (self.current_webhook_is_get || self.current_webhook_is_post);

                        let button_widget = egui::Button::new("Save Webhook configuration");                    
                        
                        let button_response = ui.add_enabled(enabled, button_widget).on_hover_cursor(egui::CursorIcon::PointingHand);

                        if button_response.clicked(){
                            if !self.current_webhook_url.is_empty(){
                                let fixed_payload = self.current_webhook_payload.clone().replace("\t", "").replace("\n", "").replace("\"", "'");
                                let success = block_on(connections::http::PretendoHttpClient::add_webhook(&self.current_pretendo.id.unwrap(), &self.current_webhook_url, &fixed_payload, &self.current_webhook_http_verb.clone().unwrap(), &self.current_webhook_delay)).unwrap();
                                logger::debug::println!("attempted to save webhook, success: {:?}", success);
                                should_display = false;
                                if success{
                                    self.current_webhook_url = String::new();
                                    self.current_webhook_payload = String::new();
                                    self.current_webhook_is_get = false;
                                    self.current_webhook_is_post = false;
                                    self.current_webhook_http_verb = None;
                                    self.current_webhook_delay = 0;
                                }
                            }
                        }
                    });
                    //close window event
                    if webhook_window.is_none(){
                        self.current_webhook_url = String::new();
                        self.current_webhook_payload = String::new();
                        self.current_webhook_is_get = false;
                        self.current_webhook_is_post = false;
                        self.current_webhook_http_verb = None;
                        self.current_webhook_delay = 0;
                    }
                }
                self.display_configure_webhooks &= should_display;
            }
        }
    }
}