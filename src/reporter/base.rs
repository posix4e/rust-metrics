pub trait Reporter: Send + Sync {
    fn get_unique_reporter_name(&self) -> &'static str;
}
