use sqlite::{Error, State};

#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    pub description: String,
    pub photo: String,
    pub google_maps: String,
    pub location_x: f64,
    pub location_y: f64,
    pub caffee_name: String,
    pub address: String,
}

pub async fn base_data() -> Result<Vec<Base>, Error> {
    let connection = sqlite::open("db.sql")?;
    let query = "SELECT * FROM museums";
    let mut statement = connection.prepare(query)?;

    let mut base_filds: Vec<Base> = vec![];

    while let Ok(State::Row) = statement.next() {
        let temp_sctruct = Base {
            description: statement.read::<String, _>("description")?,
            photo: statement.read::<String, _>("photo")?,
            google_maps: statement.read::<String, _>("google_maps")?,
            location_x: statement.read::<f64, _>("location_x")?,
            location_y: statement.read::<f64, _>("location_y")?,
            caffee_name: statement.read::<String, _>("caffee_name")?,
            address: statement.read::<String, _>("address")?,
        };
        base_filds.push(temp_sctruct)
    }

    Ok(base_filds)
}
