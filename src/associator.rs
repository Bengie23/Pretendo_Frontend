use crate::data::entities::{Pretendo, PretendoElement};
use crate::app::MyApp;
use futures::executor::block_on;

pub trait Associator {
    // fn get_associated_pretendos(&self, domain: String) -> Vec<Pretendo>;
    fn populate_data(&mut self);
}

impl Associator for MyApp {
    // fn get_associated_pretendos(&self, domain: String) -> Vec<Pretendo> {
    //     let mut result = Vec::new();
    //     let elements = self.data.clone();
    //     for element in elements {
    //         if element.domain == domain {
    //             result = element.pretendos;
    //         }
    //     }
    //     result
    // }

    fn populate_data(&mut self) {
        if self.backend_alive {
            println!("populating data");
            let mut pretendos = Vec::new();
            let domains_call = crate::connections::http::PretendoHttpClient::get_domains();
            let domains = block_on(domains_call);
            for domain in domains {
                pretendos.push(PretendoElement { domain: domain, pretendos: [Pretendo::new()].to_vec() });
            }
            self.data = pretendos;
        }
    }
}