mod connections;
use eframe::egui::{self, Button, CentralPanel, SidePanel, TextEdit, TopBottomPanel, Visuals};
use connections::http::PretendoHttpClient;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use egui::Ui;
use regex::Regex;

#[tokio::main]
async fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder::default().with_resizable(true).with_maximize_button(false).with_inner_size([1000.0, 600.0]).with_min_inner_size([840.0,600.0]),        
        ..Default::default()
    };
    
    eframe::run_native(
        "Pretendo App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(MyApp::default().theme);
            Ok(Box::<MyApp>::default())
        }),
    )
}


#[derive(Clone, Debug)]
struct MyApp {
    theme: Visuals,
    data: Vec<PretendoElement>,
    current_domain: String,
    current_pretendo: Pretendo,
    display_new_domain: bool,
    display_new_pretendo: bool,
    pretendos_in_current_domain: Vec<Pretendo>
}

pub trait Associator {
    fn get_associated_pretendos(&self, domain: String) -> Vec<Pretendo>;
}

impl Associator for MyApp {
    
    fn get_associated_pretendos(&self, domain: String) -> Vec<Pretendo> {
        let mut result = Vec::new();
        let elements = self.data.clone();
        for element in elements {
            if element.domain == domain{
                result = element.pretendos;
            }
        }
        return result;

    }

    
}



#[derive(Clone)]
#[derive(Debug)]
struct PretendoElement {
    domain: String,
    pretendos: Vec<Pretendo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct Pretendo {
    id: Option<i32>,
    path: String,
    return_object: String,
    #[serde(deserialize_with = "deserialize_string_from_number")]
    status_code:  String,
    name: String,
}

impl Pretendo {
    fn new() -> Self {
        Self {
            id: None,
            path: String::default(),
            return_object: String::default(),
            status_code: String::default(),
            name: String::default(),
        }
    }
}
impl Default for MyApp {
    fn default() -> Self {
        let mut pretendos = Vec::new();
        let domains_call = PretendoHttpClient::get_domains();
        let domains = block_on(domains_call);
        for domain in domains {
            pretendos.push(PretendoElement { domain: domain, pretendos: [Pretendo::new()].to_vec()})
        }
        Self {
            theme: Visuals::dark(),
            data: pretendos,
            current_domain: String::default(),
            display_new_pretendo: false,
            current_pretendo: Pretendo::new(),
            display_new_domain: false,
            pretendos_in_current_domain : Vec::new(),
        }
    }
}

impl eframe::App for MyApp {     
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        
        self.configure_new_domain_window(ctx);

        self.display_header(ctx);

        self.display_domains_list(ctx);
        
        self.display_pretendos_list(ctx);
        
    }
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

pub trait PretendoHeader {
    fn display_header(&mut self, ctx: &egui::Context);
}

impl PretendoHeader for MyApp {
    fn display_header(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel_0").show(ctx, |ui| {
            ui.heading("Pretendo App    🐼");
            ui.label(r#"What's a pretendo?
            - A pretendo is a local mock server that simulates the behavior of an actual server by returning customized responses to specific requests.
In order to create 'pretendo's you first need to register a domain like 'mydomain.com'. Every domain can have multiple pretendos."#);
        });
    }
}

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

pub trait DomainsList {
    fn display_domains_list(&mut self, ctx: &egui::Context);    
}

impl DomainsList for MyApp {
    fn display_domains_list(&mut self, ctx: &egui::Context) {
        if !self.display_new_domain{
            let domains: Vec<String> = self.data.clone().into_iter().map(|element| element.domain).collect();
            let lengths: Vec<i32> = domains.clone().into_iter().map(|element| element.len() as i32).collect();
            let max = lengths.iter().max().unwrap();
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

pub trait PretendosList {
    fn display_pretendos_list(&mut self, ctx: &egui::Context);    
}

impl PretendosList for MyApp{
    fn display_pretendos_list(&mut self, ctx: &egui::Context) {
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
                        }
                    }
                    let button = ui.button("➕ New Pretendo").on_hover_cursor(egui::CursorIcon::PointingHand);
                    if button.clicked() {
                        self.display_new_pretendo = true;
                        self.current_pretendo = Pretendo::new();
                    }
    
                });
            });
        }
        if self.display_new_pretendo {            
            CentralPanel::default().show(ctx, |ui| {
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
                        ui.add_sized(egui::vec2(ui.available_size().x,20.0), egui::TextEdit::singleline(&mut self.current_domain.clone()));
                    });
                    
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(sized_text("Name:", 17)));
                    ui.add_sized(egui::vec2(ui.available_size().x, 20.0), egui::TextEdit::singleline(&mut self.current_pretendo.name));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(sized_text("Path:", 20)));
                    ui.add_sized(egui::vec2(ui.available_size().x, 20.0), egui::TextEdit::singleline(&mut self.current_pretendo.path));
                });
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.add(egui::Label::new(sized_text("Return Object:", 1)));
                    
                    let win_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
                    ui.add_sized(egui::vec2(ui.available_size().x, ui.available_size().y + (win_rect.height() - 400.0)), TextEdit::multiline(&mut self.current_pretendo.return_object));
                    
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
                ui.add_sized(egui::vec2(220.0, 30.0), |ui: &mut Ui| {
                    let element = Button::new("💾 Save pretendo");
                    let output = ui.add_enabled(self.current_pretendo.id.is_none(), element).on_hover_cursor(egui::CursorIcon::PointingHand);
                    if output.clicked() {
                        let replaced_return_object = self.current_pretendo.return_object.clone().replace("\n", "").replace("\"", "'");
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
            });
        }
    }
}