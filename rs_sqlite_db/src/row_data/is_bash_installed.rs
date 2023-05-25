use super::_types::RowArgs;

pub fn data() -> RowArgs {
    RowArgs::from(
        "cmd_001",
        "Command 001",
        "Description for 'Command 001'",
        vec!["cmd", "001"],
    )
}
