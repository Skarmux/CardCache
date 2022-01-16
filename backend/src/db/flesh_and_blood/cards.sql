CREATE OR ALTER TABLE FleshAndBlood_Cards (
    id INT NOT NULL PRIMARY KEY,
    card_id SMALLINT NOT NULL,
    class_id SMALLINT NOT NULL,
    talent_id SMALLINT,
    essence_id SMALLINT,
    card_type_id SMALLINT,
    cost SMALLINT,
    rarity_id SMALLINT NOT NULL,
    pitch SMALLINT,
    attack SMALLINT,
    defense SMALLINT,
    description VARCHAR(500),
    set_id INT NOT NULL,
    
    FOREIGN KEY (card_id) REFERENCES Cards(id),
    FOREIGN KEY (class_id) REFERENCES FleshAndBlood_Classes(id),
    FOREIGN KEY (talent_id) REFERENCES FleshAndBlood_Talents(id),
    FOREIGN KEY (essence_id) REFERENCES FleshAndBlood_Essences(id),
    FOREIGN KEY (card_type_id) REFERENCES FleshAndBlood_CardTypes(id),
    FOREIGN KEY (rarity_id) REFERENCES FleshAndBlood_Rarities(id),
    FOREIGN KEY (set_id) REFERENCES FleshAndBlood_Sets(id)
);
