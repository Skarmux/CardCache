use mobc::{ Connection, Pool };
use mobc_postgres::{ tokio_postgres, PgConnectionManager };
use std::convert::Infallible;
use tokio_postgres::NoTls;
use warp::{ http::{header, Method}, Filter, Rejection };
use entity::sea_orm;

pub use::entity::game;
pub use::entity::game::Entity as Game;

mod error;
mod handler;

type Result<T> = std::result::Result<T, Rejection>;
type DBCon = Connection<PgConnectionManager<NoTls>>;
type DBPool = Pool<PgConnectionManager<NoTls>>;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut opt = ConnectOptions::new("postgres://postgres@host.docker.internal:7878/postgres".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    let cards = warp::path("cards");
    let games = warp::path("games");
    let game_cards = warp::path!("games" / i32 / "cards");

    // MEMO: The Order of route declaration is important!

    let card_routes = cards
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handler::fetch_card_handler)
        .or(
            cards
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::list_cards_handler)
        )
        .or(
            game_cards
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::create_card_handler)
        )
        .or(
            cards
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_card_handler)
        );

    let game_routes = games
        .and(warp::get())
        .and(warp::path::param())
        .and(with_db(db.clone()))
        .and_then(handler::fetch_game_handler)
        .or(
            games
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::list_games_handler)
        )
        .or(
            game_cards
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::list_game_cards_handler)
        )
        .or(
            games
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::create_game_handler)
        );

    let routes =
        card_routes
        .or(game_routes)
        .recover(error::handle_rejection)
        .with(
            warp::cors()
                .allow_credentials(true)
                .allow_methods(&[
                    Method::OPTIONS,
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT ])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .expose_headers(vec![header::LINK])
                .max_age(300)
                .allow_any_origin(),
        );

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(db: DBPool) -> impl Filter<Extract = (DBPool,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
