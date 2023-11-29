PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS "countries" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    continent TEXT
);
INSERT INTO countries VALUES (1, 'Poland','Europe');
INSERT INTO countries VALUES (2, 'United Kingdom','Europe');
INSERT INTO countries VALUES (3, 'Indonesia','Asia');
INSERT INTO countries VALUES (4, 'Thailand','Asia');
INSERT INTO countries VALUES (5, 'Botswana','Africa');
INSERT INTO countries VALUES (6, 'South Africa','Africa');
INSERT INTO countries VALUES (7, 'France', 'Europe');
INSERT INTO countries VALUES (8, 'Japan', 'Asia');
INSERT INTO countries VALUES (9, 'Brazil', 'South America');
INSERT INTO countries VALUES (10, 'Canada', 'North America');
INSERT INTO countries VALUES (11, 'Australia', 'Australia');
INSERT INTO countries VALUES (12, 'Germany', 'Europe');
INSERT INTO countries VALUES (13, 'Kenya', 'Africa');
INSERT INTO countries VALUES (14, 'Mexico', 'North America');
INSERT INTO countries VALUES (15, 'India', 'Asia');
INSERT INTO countries VALUES (16, 'New Zealand', 'Oceania');

CREATE TABLE IF NOT EXISTS "cities" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country_id INTEGER,
    FOREIGN KEY (country_id) REFERENCES countries(id)
);
INSERT INTO cities (name, country_id) VALUES ('Warsaw', 1);
INSERT INTO cities (name, country_id) VALUES ('Krakow', 1);
INSERT INTO cities (name, country_id) VALUES ('London', 2);
INSERT INTO cities (name, country_id) VALUES ('Manchester', 2);
INSERT INTO cities (name, country_id) VALUES ('Jakarta', 3);
INSERT INTO cities (name, country_id) VALUES ('Bali', 3);
INSERT INTO cities (name, country_id) VALUES ('Bangkok', 4);
INSERT INTO cities (name, country_id) VALUES ('Chiang Mai', 4);
INSERT INTO cities (name, country_id) VALUES ('Johannesburg', 6);
INSERT INTO cities (name, country_id) VALUES ('Cape Town', 6);
INSERT INTO cities (name, country_id) VALUES ('Paris', 7);
INSERT INTO cities (name, country_id) VALUES ('Lyon', 7);
INSERT INTO cities (name, country_id) VALUES ('Tokyo', 8);
INSERT INTO cities (name, country_id) VALUES ('Kyoto', 8);
INSERT INTO cities (name, country_id) VALUES ('Rio de Janeiro', 9);
INSERT INTO cities (name, country_id) VALUES ('Sao Paulo', 9);
INSERT INTO cities (name, country_id) VALUES ('Toronto', 10);
INSERT INTO cities (name, country_id) VALUES ('Vancouver', 10);
INSERT INTO cities (name, country_id) VALUES ('Sydney', 11);
INSERT INTO cities (name, country_id) VALUES ('Melbourne', 11);


DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('countries',7);
COMMIT;
