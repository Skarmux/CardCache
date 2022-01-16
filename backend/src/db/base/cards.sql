CREATE TABLE IF NOT EXISTS Cards (
    id INT NOT NULL PRIMARY KEY,
    game_id SMALLINT,
    illustrator_id INT,
    name VARCHAR(50) NOT NULL,
    code VARCHAR(10) NOT NULL,
    
    FOREIGN KEY (game_id) REFERENCES Games(id),
    FOREIGN KEY (illustrator_id) REFERENCES Illustrators(id)
);
