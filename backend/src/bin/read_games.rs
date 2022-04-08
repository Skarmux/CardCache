#[macro_use]
extern crate backend;
extern crate diesel;

use self::backend::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use backend::schema::games::dsl::*;

    let connection = establish_connection();
    let results = games.filter(name.eq("FAB"))
        .limit(5)
        .load::<Game>(&connection)
        .expect("Error loading games");
    
    println!("Displaying {} games", results.len());
    for game in results {
        println!("{}",game.name);
        println!("-----------\n");
    }
}