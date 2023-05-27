use super::_types::RowData;

pub fn row_data() -> RowData {
    RowData::new(
        "cmd_003",
        "Command 003",
        "Description for 'Command 003'",
        vec!["cmd", "003"],
    )
}
