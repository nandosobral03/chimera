-- Your SQL goes here-- Your SQL goes here

CREATE TABLE user_stats (
    user_id INT NOT NULL REFERENCES users(id),
    total_games INT NOT NULL DEFAULT 0,
    total_wins INT NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id)
);


CREATE TABLE user_day_stats (
    user_id INT NOT NULL REFERENCES users(id),
    day VARCHAR(255) NOT NULL REFERENCES games(day),
    status VARCHAR(255) NOT NULL, 
    board VARCHAR(511) NOT NULL,
    last_move VARCHAR(255),
    PRIMARY KEY (user_id, day)
);


CREATE TABLE guest_day_stats (
    guest_id VARCHAR(255),
    day VARCHAR(255) NOT NULL REFERENCES games(day),
    status VARCHAR(255) NOT NULL, 
    board VARCHAR(511) NOT NULL,
    PRIMARY KEY (guest_id, day)
);


CREATE TABLE guests(
    id VARCHAR(255) NOT NULL,
    games_played INT NOT NULL DEFAULT 0,
    wins INT NOT NULL DEFAULT 0,
    PRIMARY KEY (id)
);

CREATE TABLE day_stats(
    day VARCHAR(255) NOT NULL REFERENCES games(day),
    total_games INT NOT NULL DEFAULT 0,
    total_wins INT NOT NULL DEFAULT 0,
    aggregated_board_stats VARCHAR(511) NOT NULL,
    PRIMARY KEY (day)
);
