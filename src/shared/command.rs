pub trait CommandInfo {
    fn caller(&self) -> &'static str;
}
