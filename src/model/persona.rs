use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::personas;

#[table_name = "personas"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Persona {
    pub id: Option<i32>,
    pub name: String,
    pub lastname: String,
    pub country: String,
    pub province: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub sex: String,
    pub comment1: Option<String>,
    pub comment2: Option<String>,
    pub comment3: Option<String>,
    pub age: Option<i32>,
    pub mozilla_news: i32
}

impl Persona {
    pub fn create(persona: Persona, connection: &MysqlConnection) -> Persona {
        diesel::insert_into(personas::table)
            .values(&persona)
            .execute(connection)
            .expect("Error creating new persona");
        personas::table.order(personas::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Persona> {
        personas::table.order(personas::id).load::<Persona>(connection).unwrap()
    }
    pub fn select(id: i32, connection: &MysqlConnection) -> Persona {
        personas::table.find(id).first::<Persona>(connection).unwrap()
    }

    pub fn update(id: i32, persona: Persona, connection: &MysqlConnection) -> bool {
        diesel::update(personas::table.find(id)).set(&persona).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(personas::table.find(id)).execute(connection).is_ok()
    }
}