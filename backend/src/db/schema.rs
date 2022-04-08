table! {
    cards (id) {
        id -> Int4,
        name -> Varchar,
        code -> Varchar,
        game_id -> Int4,
        illustrator_id -> Nullable<Int4>,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        acronym -> Varchar,
    }
}

table! {
    illustrators (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(cards -> games (game_id));
joinable!(cards -> illustrators (illustrator_id));

allow_tables_to_appear_in_same_query!(
    cards,
    games,
    illustrators,
);
