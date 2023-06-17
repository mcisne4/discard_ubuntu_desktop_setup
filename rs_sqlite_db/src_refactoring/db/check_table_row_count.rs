use rusqlite::Connection;

const ROWS_COUNT: usize = 0;

pub fn check_table_row_count(conn: &Connection) -> Result<bool, String> {
    let row_count = conn.query_row("SELECT COUNT(*) FROM command_list", [], |row| {
        let row_count = row.get::<usize, usize>(0);
        row_count
    });

    match row_count {
        Ok(count) => Ok(count == ROWS_COUNT),
        Err(e) => Err(e.to_string()),
    }
}
