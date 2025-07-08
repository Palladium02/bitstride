pub(crate) struct PersistenceService {
    url: String,
}

impl PersistenceService {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_owned(),
        }
    }
}
