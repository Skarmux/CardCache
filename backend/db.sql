-- DROP TABLE Cards;
-- DROP TABLE Games;
-- DROP TABLE Illustrators;

CREATE TABLE IF NOT EXISTS Games (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    acronym VARCHAR(10) NOT NULL
);

CREATE TABLE IF NOT EXISTS Illustrators (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS Cards (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    code VARCHAR(10) NOT NULL,
    game_id INT NOT NULL,
    illustrator_id INT,
    
    CONSTRAINT fk_cards_games_id FOREIGN KEY (game_id) REFERENCES Games(id),
    CONSTRAINT fk_cards_illustrators_id FOREIGN KEY (illustrator_id) REFERENCES Illustrators(id)
);
