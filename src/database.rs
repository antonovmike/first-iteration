use sqlite::{Error, State};

#[derive(Debug, Clone, PartialEq)]
pub struct CoffeeHouse {
    pub description: String,
    pub photo: String,
    pub google_map: String,
    pub location_x: f64,
    pub location_y: f64,
    pub caffee_name: String,
    pub address: String,
}

pub async fn kofe_list() -> Result<Vec<CoffeeHouse>, Error> {
    let connection = sqlite::open("db.sql")?;
    let query = "SELECT * FROM coffee";
    let mut statement = connection.prepare(query)?;

    let mut base_filds: Vec<CoffeeHouse> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = CoffeeHouse {
            description: statement.read::<String, _>("description")?,
            photo: statement.read::<String, _>("photo")?,
            google_map: statement.read::<String, _>("google_map")?,
            location_x: statement.read::<f64, _>("location_x")?,
            location_y: statement.read::<f64, _>("location_y")?,
            caffee_name: statement.read::<String, _>("caffee_name")?,
            address: statement.read::<String, _>("address")?,
        };
        base_filds.push(temp_sctruct)
    }

    Ok(base_filds)
}
