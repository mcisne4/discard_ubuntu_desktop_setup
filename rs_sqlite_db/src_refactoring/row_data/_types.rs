pub struct RowData {
    pub command: String,
    pub title: String,
    pub description: String,
    pub keywords: String,
    pub use_count: usize,
}
impl RowData {
    pub fn new(command: &str, title: &str, description: &str, keywords: Vec<&str>) -> Self {
        Self {
            command: command.to_owned(),
            title: title.to_owned(),
            description: description.to_owned(),
            keywords: keywords.join("|"),
            use_count: 0,
        }
    }

    pub fn as_args(self) -> (String, String, String, String, usize) {
        (
            self.command,
            self.title,
            self.description,
            self.keywords,
            self.use_count,
        )
    }
}
