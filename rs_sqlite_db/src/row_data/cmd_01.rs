use super::_types::RowData;

pub fn row_data() -> RowData {
    RowData::new(
        "cmd_001",
        "Command 001",
        "Description for 'Command 001'",
        vec!["cmd", "001"],
    )
}
