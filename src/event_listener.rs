pub mod utils{
    use crate::{app::MyApp, associator::Associator};

    pub trait EventListener {
        fn listen_events(&mut self);
    }
    impl EventListener for MyApp {
        fn listen_events(&mut self) {
            if let Ok(msg) = self.rx_gui.try_recv() {
                if msg == "alive"{
                    self.backend_alive = true;
                    self.populate_data();
                }
                if msg == "dead" {
                    self.running_backend_ping_pong = false;
                }
            }
        }
    }
}