use serde::{ Deserialize, Serialize };

// GAME

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub acronym: String
}

impl Game {
    pub fn of(game: GameResponse) -> Game {
        Game {
            id: game.id,
            name: game.name,
            acronym: game.acronym
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GameRequest {
    pub name: String,
    pub acronym: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct GameResponse {
    pub id: i32,
    pub name: String,
    pub acronym: String
}

impl GameResponse {
    pub fn of(game: Game) -> GameResponse {
        GameResponse {
            id: game.id,
            name: game.name,
            acronym: game.acronym
        }
    }
}

// ILLUSTRATOR

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Illustrator {
    pub id: i32,
    pub name: String
}

impl Illustrator {
    pub fn of(illustrator: IllustratorResponse) -> Illustrator {
        Illustrator {
            id:   illustrator.id,
            name: illustrator.name
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IllustratorRequest {
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IllustratorResponse {
    pub id: i32,
    pub name: String
}

impl IllustratorResponse {
    pub fn of(illustrator: Illustrator) -> IllustratorResponse {
        IllustratorResponse {
            id:   illustrator.id,
            name: illustrator.name
        }
    }
}

// CARD

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Card {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub game_id: i32,
    pub illustrator_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CardRequest {
    pub name: String,
    pub code: String,
    pub game_id: i32,
    pub illustrator_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CardResponse {
    pub id:   i32,
    pub name: String,
    pub code: String,
    pub game_id: i32,
    pub illustrator_id: Option<i32>,
}

impl CardResponse {
    pub fn of(card: Card) -> CardResponse {
        CardResponse {
            id:   card.id,
            name: card.name,
            code: card.code,
            game_id: card.game_id,
            illustrator_id: card.illustrator_id,
        }
    }
}
