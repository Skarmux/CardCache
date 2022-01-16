use super::{ get_db_con, Result };
use crate::{ error::Error::*, DBPool };
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "Cards";
const SELECT_FIELDS: &str = "id, name, code, game_id, illustrator_id";

fn row_to_card(row: &Row) -> Card {
    let id:      i32        = row.get(0);
    let name:    String     = row.get(1);
    let code:    String     = row.get(2);
    let game_id: i32        = row.get(3);
    let illustrator_id: Option<i32> = row.get(4);
    Card { id, name, code, game_id, illustrator_id }
}

pub async fn create(db_pool: &DBPool, game_id: i32, body: CardRequest) -> Result<Card> {
    let con = get_db_con(db_pool).await?;
    let query = format!( "INSERT INTO {} (name, code, game_id, illustrator_id) VALUES ($1, $2, $3, $4) RETURNING *", TABLE );
    let row = con
        .query_one(query.as_str(), &[&body.name, &body.code, &body.game_id, &body.illustrator_id])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_card(&row))
}

pub async fn fetch_all(db_pool: &DBPool) -> Result<Vec<Card>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_card(&r)).collect())
}

pub async fn fetch_all_by_game(db_pool: &DBPool, game_id: i32) -> Result<Vec<Card>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE game_id = $1", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[&game_id]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_card(&r)).collect())
}

pub async fn fetch_one(db_pool: &DBPool, id: i32) -> Result<Card> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {} WHERE id = $1", SELECT_FIELDS, TABLE);
    let row = con
        .query_one(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)?;

    Ok(row_to_card(&row))
}

pub async fn delete(db_pool: &DBPool, id: i32) -> Result<u64> {
    let con = get_db_con(db_pool).await?;
    let query = format!("DELETE FROM {} WHERE id = $1", TABLE);
    con.execute(query.as_str(), &[&id])
        .await
        .map_err(DBQueryError)
}
