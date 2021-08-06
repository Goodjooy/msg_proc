use msg_chain::MessageChain;


pub mod chain_builder;
pub mod resouce;
pub mod chain_handle;

type BulidTarget = Vec<Box<dyn MessageChain>>;
#[macro_export]
macro_rules! chain_generate {
    (xml : $s:expr) => {
        Box::new(chain::msg_chain::Xml {
            xml: $s.to_string(),
        })
    };
    (json : $s:expr) => {
        Box::new(chain::msg_chain::Json {
            json: $s.to_string(),
        })
    };
    (app : $s:expr) => {
        Box::new(chain::msg_chain::App {
            content: $s.to_string(),
        })
    };
}

