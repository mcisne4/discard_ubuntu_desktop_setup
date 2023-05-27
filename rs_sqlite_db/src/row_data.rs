mod _types;
pub use _types::RowData;

mod cmd_01;
mod cmd_02;
mod cmd_03;
mod cmd_04;

pub fn get_row_data() -> Vec<RowData> {
    let rows: Vec<RowData> = vec![
        cmd_01::row_data(),
        cmd_02::row_data(),
        cmd_03::row_data(),
        cmd_04::row_data(),
    ];
    rows
}
