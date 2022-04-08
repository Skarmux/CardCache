// use crate::{ db, DBPool, Result };
// use common::*;
// use warp::{ http::StatusCode, reject, reply::json, Reply };

// // CARDS

// pub async fn create_card_handler(game_id: i32, body: CardRequest, db_pool: DBPool) -> Result<impl Reply> {
//     Ok(json(&CardResponse::of(
//         db::cards::create(&db_pool, game_id, body)
//             .await
//             .map_err(reject::custom)?,
//     )))
// }

// pub async fn list_cards_handler(db_pool: DBPool) -> Result<impl Reply> {
//     let cards = db::cards::fetch_all(&db_pool)
//         .await
//         .map_err(reject::custom)?;
//     Ok(json::<Vec<_>>(
//         &cards.into_iter().map(CardResponse::of).collect(),
//     ))
// }

// pub async fn fetch_card_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
//     let card = db::cards::fetch_one(&db_pool, id)
//         .await
//         .map_err(reject::custom)?;

//     Ok(json(&CardResponse::of(card)))
// }


// // GAMES

// pub async fn create_game_handler(body: GameRequest, db_pool: DBPool) -> Result<impl Reply> {
//     let game = db::games::create(&db_pool, body)
//         .await
//         .map_err(reject::custom)?;

//     Ok(json(&GameResponse::of(game)))
// }

// pub async fn list_games_handler(db_pool: DBPool) -> Result<impl Reply> {
//     let games = db::games::fetch_all(&db_pool)
//         .await
//         .map_err(reject::custom)?;

//     Ok(json::<Vec<_>>(&games.into_iter().map(GameResponse::of).collect()))
// }

// pub async fn fetch_game_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
//     let game = db::games::fetch_one(&db_pool, id)
//         .await
//         .map_err(reject::custom)?;

//     Ok(json(&GameResponse::of(game)))
// }

// pub async fn list_game_cards_handler(game_id: i32, db_pool: DBPool) -> Result<impl Reply> {
//     let cards = db::cards::fetch_all_by_game(&db_pool, game_id)
//         .await
//         .map_err(reject::custom)?;

//     Ok(json::<Vec<_>>(&cards.into_iter().map(CardResponse::of).collect()))
// }

// pub async fn delete_card_handler(id: i32, db_pool: DBPool) -> Result<impl Reply> {
//     db::cards::delete(&db_pool, id)
//         .await
//         .map_err(reject::custom)?;

//     Ok(StatusCode::OK)
// }
