pub trait ForeignString {
    fn normalize_id(&self) -> Self;
}

impl ForeignString for String {
    fn normalize_id(&self) -> Self {
        self.trim()
            .to_lowercase()
            .chars()
            .map(|c| if c.is_whitespace() { '_' } else { c })
            .collect()
    }
}
