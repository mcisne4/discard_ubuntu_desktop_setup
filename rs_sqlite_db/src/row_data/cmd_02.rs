use super::_types::RowData;

pub fn row_data() -> RowData {
    RowData::new(
        "cmd_002",
        "Command 002",
        "Description for 'Command 002'",
        vec!["cmd", "002"],
    )
}
