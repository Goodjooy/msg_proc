use msg_chain::MessageChain;


mod rev;
mod send;
mod impls;

pub trait Sender {
    fn get_sender_id(&self) -> &u64;
    fn get_sender_permission(&self) -> Option<&Permission> {
        None
    }
    fn get_sender_name(&self) -> Option<&String> {
        None
    }
    fn get_sender_market(&self) -> Option<&String> {
        None
    }

    fn get_group_from(&self) -> Option<&u64> {
        None
    }
    fn get_group_name(&self) -> Option<&String> {
        None
    }

    fn get_self_permession(&self) -> Option<&Permission> {
        None
    }

    fn get_platform(&self) -> Option<&Platform> {
        None
    }
}

pub struct MessageRev {
    pub msg_type: String,
    pub sender: Box<dyn Sender>,
    pub chain: Vec<Box<dyn MessageChain>>,
}

pub enum Permission {
    Owner,
    Member,
    Admin,
}

pub enum Platform {
    Mobile,
}

