pub mod pretendo_ui{
    use eframe::egui::{self, TopBottomPanel};

    use crate::app::MyApp;

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
}