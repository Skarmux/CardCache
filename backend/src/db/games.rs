use crate::Result;
use common::*;
use warp::{ http::StatusCode, reject, reply::json, Reply };
use self::models::{ Game, CreateGame };

pub async fn create_game<'a>(conn: &PgConnection, name: &'a str, acronym: &'a str) -> Game
{
    use schema::games;

    let new_game = CreateGame {
        name: name,
        acronym: acronym,
    }

    diesel::insert_into( posts::table )
        .values(&new_post)
        .get_result(conn) // or .execute() if no interest in result
        .expect("Error saving new game")
}

pub async fn fetch_all() -> Result<impl Reply>
{
    use backend::schema::games::dsl::*;

    let connection = establish_connection();
    let results = games
        .load::<Game>(&connection)
        .expect("Error loading games");
    results
}
