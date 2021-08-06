pub enum ResouceSrc<'a> {
    Id(&'a str),
    Path(&'a str),
    Url(&'a str),
    Base64(&'a str),
}

impl<'a> ResouceSrc<'a> {
    pub fn id(id: &'a str) -> Self {
        Self::Id(id)
    }
    pub fn path(path: &'a str) -> Self {
        Self::Path(path)
    }
    pub fn url(url: &'a str) -> Self {
        Self::Url(url)
    }
    pub fn base64(base: &'a str) -> Self {
        Self::Base64(base)
    }

    pub fn get_value(self) -> String {
        match self {
            ResouceSrc::Id(s)
            | ResouceSrc::Path(s)
            | ResouceSrc::Url(s)
            | ResouceSrc::Base64(s) => s.to_string(),
        }
    }
}
