use std::collections::HashMap;

#[derive(Debug)]
pub struct Application {
    pub path: String,
    pub alias: Option<String>,
    pub env: HashMap<String, String>,
}

impl Application {
    pub fn new<S>(path: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            path: path.into(),
            alias: None,
            env: HashMap::new(),
        }
    }
}

impl AsRef<str> for Application {
    fn as_ref(&self) -> &str {
        match &self.alias {
            Some(alias) => alias.as_ref(),
            None => self.path.as_ref(),
        }
    }
}
