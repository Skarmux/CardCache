-- For example `Monarch (MON)`
CREATE TABLE IF NOT EXISTS Sets (
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
    set_id INT NOT NULL,
    illustrator_id INT,
    
    CONSTRAINT fk_cards_sets_id FOREIGN KEY (set_id) REFERENCES Sets(id),
    CONSTRAINT fk_cards_illustrators_id FOREIGN KEY (illustrator_id) REFERENCES Illustrators(id)
);
