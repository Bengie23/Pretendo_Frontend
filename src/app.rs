use crate::event_listener::utils::EventListener;
use crate::pretendo_domain_list::pretendo_ui::DomainsList;
use crate::pretendo_header::pretendo_ui::PretendoHeader;
use crate::pretendo_loader::pretendo_ui::PretendoLoader;
use crate::pretendo_new_domain_window::pretendo_ui::NewDomainWindow;
use crate::pretendo_new_webhook_window::pretendo_ui::NewWebhookWindow;

use crate::data::entities::{Pretendo, PretendoElement, Webhook, HttpVerbs};
use crate::pretendo_pretendos_list::pretendo_ui::PretendosList;
use eframe::egui;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::executor::block_on;

#[derive(Debug)]
pub struct MyApp {
    pub backend_alive: bool,
    pub data: Vec<PretendoElement>,
    pub current_domain: String,
    pub current_pretendo: Pretendo,
    pub display_new_domain: bool,
    pub display_new_pretendo: bool,
    pub pretendos_in_current_domain: Vec<Pretendo>,
    pub display_configure_webhooks: bool,
    pub webhooks_in_current_pretendo: Vec<Webhook>,
    pub current_webhook_url: String,
    pub current_webhook_payload: String,
    pub current_webhook_http_verb: Option<HttpVerbs>,
    pub current_webhook_is_get: bool,
    pub current_webhook_is_post: bool,
    pub running_backend_ping_pong: bool,
    pub rx_gui: UnboundedReceiver<String>,
    pub tx_gui: UnboundedSender<String>,
    pub current_webhook_delay: i32,
}

impl MyApp {
    pub fn new(rx: UnboundedReceiver<String>, tx: UnboundedSender<String>) -> Self {
        Self {
            backend_alive: block_on(crate::connections::http::PretendoHttpClient::backend_alive()) || false,
            data: Vec::new(),
            current_domain: String::default(),
            display_new_pretendo: false,
            current_pretendo: Pretendo::new(),
            display_new_domain: false,
            pretendos_in_current_domain: Vec::new(),
            webhooks_in_current_pretendo: Vec::new(),
            display_configure_webhooks: false,
            current_webhook_url: String::default(),
            current_webhook_payload: String::default(),
            current_webhook_http_verb: None,
            current_webhook_is_get: false,
            current_webhook_is_post: false,
            running_backend_ping_pong: false,
            tx_gui: tx,
            rx_gui: rx,
            current_webhook_delay: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.backend_alive && self.data.len() > 0 {
            true => {
                self.configure_new_webhook_window(ctx);
                self.configure_new_domain_window(ctx);
                self.display_header(ctx);
                self.display_domains_list(ctx);
                self.display_pretendos_list(ctx);
            },
            false => {
                self.display_loader(ctx);
                self.listen_events();
            },
        }
    }
}