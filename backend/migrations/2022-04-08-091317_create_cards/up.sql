-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Cards (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    code VARCHAR(10) NOT NULL,
    game_id INT NOT NULL,
    illustrator_id INT,
    
    CONSTRAINT fk_cards_games_id FOREIGN KEY (game_id) REFERENCES Games(id),
    CONSTRAINT fk_cards_illustrators_id FOREIGN KEY (illustrator_id) REFERENCES Illustrators(id)
);