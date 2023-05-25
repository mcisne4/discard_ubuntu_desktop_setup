mod _types;
use _types::{Row, Rows};

mod is_bash_installed;

pub fn get_row_data() -> Vec<Row> {
    let rows = Rows::from_args(vec![
        //
        is_bash_installed::data(),
    ]);

    rows.rows
}
