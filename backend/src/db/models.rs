use diesel::{ Queryable, Insertable };

// Order of properties must match order of table columns in schema.rs!

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub acronym: String,
}

#[derive(Queryable)]
pub struct Illustrator {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Card {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub game_id: i32,
    pub illustrator_id: Option<i32>,
}



#[derive(Insertable)]
#[table_name="games"]
pub struct CreateGame<'a> {
    pub name: &'a str,
    pub acronym: &'a str,
}
