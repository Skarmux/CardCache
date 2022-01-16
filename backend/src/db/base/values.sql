CREATE OR ALTER TABLE Values (
    id INT NOT NULL PRIMARY KEY,
    card_id INT,
    variant_id SMALLINT,
    edition_id SMALLINT,
    value FLOAT NOT NULL,
    
    FOREIGN KEY (card_id) REFERENCES Cards(id),
    FOREIGN KEY (variant_id) REFERENCES Variants(id)
    FOREIGN KEY (edition_id) REFERENCES Editions(id)
);
