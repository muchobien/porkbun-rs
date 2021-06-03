use std::borrow::Cow;

#[derive(Clone)]
pub struct Auth {
    pub apikey: Cow<'static, str>,
    pub secretapikey: Cow<'static, str>,
}

impl Auth {
    pub fn new(key: Cow<'static, str>, secret: Cow<'static, str>) -> Self {
        Self {
            apikey: key,
            secretapikey: secret,
        }
    }
}
