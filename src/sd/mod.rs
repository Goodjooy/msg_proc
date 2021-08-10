use crate::Sender;

pub mod impls;
pub mod send_body;
pub mod utils;
pub mod msg_send;
pub trait SendTarget {
    fn target_id(&self) -> u64;
    fn target_group(&self) -> Option<u64>;
}

impl SendTarget for dyn Sender {
    fn target_id(&self) -> u64 {
        *self.get_sender_id()
    }

    fn target_group(&self) -> Option<u64> {
        Some(*self.get_group_from()?)
    }
}
impl SendTarget for Box<dyn Sender> {
    fn target_id(&self) -> u64 {
        *self.get_sender_id()
    }

    fn target_group(&self) -> Option<u64> {
        Some(*self.get_group_from()?)
    }
}

pub struct CustomTarget {
    id: u64,
    group: Option<u64>,
}

impl SendTarget for CustomTarget {
    fn target_id(&self) -> u64 {
        self.id
    }

    fn target_group(&self) -> Option<u64> {
        Some(self.group?)
    }
}
#[macro_export]
macro_rules! target_generate {
    ( target => $x:expr ) => {
        CustomTarget{
            id : $x as u64
            group:None
        }
    };
    ( target => $x:expr , group => $y:expr )=>{
        CustomTarget{
            id : $x as u64,
            group : Some($y as u64)
        }
    }
}
