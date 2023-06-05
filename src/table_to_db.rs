use calamine::{open_workbook_auto, DataType, Reader};
use thiserror::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open spreadsheet file: {0}")]
    TableErr(#[from] calamine::Error),

    #[error("sql error: {0}")]
    SqlError(#[from] sqlite::Error),
}

pub fn to_base() -> Result<(), Error> {
    let path = "catalog.ods";
    let mut workbook = open_workbook_auto(path)?;

    // Select the first worksheet
    let sheet_name = workbook.sheet_names()[0].to_owned();
    let range = workbook.worksheet_range(&sheet_name).unwrap();

    let mut table_content: Vec<Vec<DataType>> = vec![];

    for row in range.expect("REASON").rows() {
        table_content.push(row.to_vec());
    }

    let connection = sqlite::open("db.sql")?;
    let query = "DROP TABLE IF EXISTS coffee";
    connection.execute(query)?;
    let query = "
    CREATE TABLE IF NOT EXISTS coffee 
    (description TEXT, photo TEXT, google_map TEXT, location_x f64, location_y f64, caffee_name TEXT, address TEXT);
    ";
    connection.execute(query)?;

    for caffee in table_content {
        let desctiption = caffee[0].to_string().replace("'", "''");

        let query = format!(
            "INSERT INTO coffee VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');",
            desctiption,
            caffee[1].to_string(), // foto
            caffee[2].to_string(), // google map
            caffee[3].to_string(), // latitude
            caffee[4].to_string(), // longitude
            caffee[5].to_string(), // caffee name
            caffee[6].to_string()  // address
        );
        connection.execute(&query)?;
    }

    Ok(())
}
