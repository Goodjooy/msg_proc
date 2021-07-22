use crate::Permission;
use msg_chain::FromChainMeta;
use serde::de::Error;
use serde::Deserialize;

use crate::Platform;

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).ok_or(Error::custom(format!("{} can not into Permission", s)))
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).ok_or(Error::custom(format!("{} can not into Platform", s)))
    }
}
impl FromChainMeta for  Permission{
    fn from_chain(chain: Option<&msg_chain::ChainMeta>) -> Option<Self> {
        let s=String::from_chain(chain)?;
        Self::from_str(&s)
    }
}
impl Permission {
    pub fn from_str(s: &str) -> Option<Self> {
        match &s[..] {
            "Owner" => Some(Self::Owner),
            "Member" => Some(Self::Member),
            "Admin" => Some(Self::Admin),
            _s => None,
        }
    }
}
impl FromChainMeta for  Platform{
    fn from_chain(chain: Option<&msg_chain::ChainMeta>) -> Option<Self> {
        let s=String::from_chain(chain)?;
        Self::from_str(&s)
    }
}
impl Platform {
    pub fn from_str(s: &str) -> Option<Self> {
        match &s[..] {
            "MOBILE" => Some(Self::Mobile),
            _s => None,
        }
    }
}
