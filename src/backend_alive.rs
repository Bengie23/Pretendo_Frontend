pub mod utils{
    use eframe::egui;
    use futures::executor::block_on;

    use crate::{app::MyApp, connections::http::PretendoHttpClient, data::entities::{Pretendo, PretendoElement}, logger};

    
    pub trait BackendAlive {
        fn check_backend(& mut self, ctx: &egui::Context);
    }

    impl BackendAlive for MyApp {
        fn check_backend(& mut self, ctx: &egui::Context) {
            if !self.backend_alive && !self.running_backend_ping_pong {
                self.running_backend_ping_pong = true;
                self.data = Vec::new();
                let frame = ctx.clone();
                let tx_gui = self.tx_gui.clone();
                tokio::spawn(async move {
                    let alive = PretendoHttpClient::backend_alive().await;
                    if alive {
                        tx_gui.send("alive".to_string()).expect("Failed to send message");
                        logger::debug::println!("message sent!");
                    }
                    else {
                        tx_gui.send("dead".to_string()).expect("Failed to send message");
                        logger::debug::println!("message sent!");
                    }
                    frame.request_repaint_after_secs(3.0);
                });
            }
            if self.backend_alive && self.data.is_empty() {
                let mut pretendos = Vec::new();
                let domains_call = PretendoHttpClient::get_domains();
                let domains = block_on(domains_call);
                for domain in domains {
                    pretendos.push(PretendoElement { domain: domain, pretendos: [Pretendo::new()].to_vec()})
                }
                self.data = pretendos;
            }
        }
    }
}