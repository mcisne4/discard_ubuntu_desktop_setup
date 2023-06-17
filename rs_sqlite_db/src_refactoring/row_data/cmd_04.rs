use super::_types::RowData;

pub fn row_data() -> RowData {
    RowData::new(
        "cmd_004",
        "Command 004",
        "Description for 'Command 004'",
        vec!["cmd", "004"],
    )
}
