use super::{ get_db_con, Result };
use crate::{ error::Error::*, DBPool };
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "Games";
const SELECT_FIELDS: &str = "id, name, acronym";

fn row_to_game(row: &Row) -> Game {
    let id:      i32    = row.get(0);
    let name:    String = row.get(1);
    let acronym: String = row.get(2);
    
    Game { id, name, acronym }
}

pub async fn create(db_pool: &DBPool, body: GameRequest) -> Result<Game> {
    let con = get_db_con(db_pool).await?;
    let query = format!( "INSERT INTO {} (name, acronym) VALUES ($1, $2) RETURNING *", TABLE );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.acronym])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_game(&row))
}

pub async fn fetch_all(db_pool: &DBPool) -> Result<Vec<Game>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[])
        .await
        .map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_game(&r)).collect())
}

pub async fn fetch_one(db_pool: &DBPool, game_id: i32) -> Result<Game> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS, TABLE);
    let row = con.query_one(query.as_str(), &[&game_id])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_game(&row))
}
