pub struct RowArgs {
    pub cmd: String,
    pub cmd_name: String,
    pub description: String,
    pub keywords: Vec<String>,
}
impl RowArgs {
    pub fn from(cmd: &str, cmd_name: &str, description: &str, keywords: Vec<&str>) -> Self {
        RowArgs {
            cmd: cmd.to_owned(),
            cmd_name: cmd_name.to_owned(),
            description: description.to_owned(),
            keywords: keywords
                .into_iter()
                .map(|kword| String::from(kword))
                .collect(),
        }
    }
}

pub struct Row {
    pub id: u32,
    pub cmd: String,
    pub cmd_name: String,
    pub prev_used: u32,
    pub description: String,
    pub keywords: String,
}
impl Row {
    pub fn to_params(&self) -> (u32, String, String, u32, String,String) {
      (self.id, self.cmd, self.cmd_name, self.)
    }
}

pub struct Rows {
    pub rows: Vec<Row>,
    count: u32,
}
impl Rows {
    pub fn new() -> Self {
        Rows {
            rows: vec![],
            count: 0,
        }
    }

    pub fn from_args(row_args: Vec<RowArgs>) -> Self {
        let mut rows = Rows::new();

        for args in row_args {
            rows.rows.push(Row {
                id: rows.count,
                cmd: args.cmd,
                cmd_name: args.cmd_name,
                prev_used: 0,
                keywords: args.keywords.join("|"),
                description: args.description,
            });
            rows.count += 1;
        }

        rows
    }
}
