use rusqlite::Connection;

pub fn check_table_exists(conn: &Connection) -> Result<bool, String> {
    let name = conn.query_row(
        "SELECT name
    FROM sqlite_master
    WHERE type='table'
    AND name='command_list';
    ",
        [],
        |row| match row.get::<usize, String>(0) {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        },
    );

    match name {
        Ok(val) => Ok(val),
        Err(e) => {
            let err = e.to_string();
            if err.contains("Query returned no rows") {
                return Ok(false);
            }

            Err(err)
        }
    }
}
