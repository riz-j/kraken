PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "countries" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    continent TEXT
, is_archived BOOLEAN NOT NULL DEFAULT 0);
INSERT INTO countries VALUES(1,'Poland','Europe',0);
INSERT INTO countries VALUES(2,'United Kingdom','Europe',0);
INSERT INTO countries VALUES(3,'Indonesia','Asia',0);
INSERT INTO countries VALUES(4,'Thailand','Asia',0);
INSERT INTO countries VALUES(5,'Botswana','Africa',0);
INSERT INTO countries VALUES(6,'South Africa','Africa',0);
INSERT INTO countries VALUES(7,'France','Europe',0);
INSERT INTO countries VALUES(8,'Japan','Asia',0);
INSERT INTO countries VALUES(9,'Brazil','South America',0);
INSERT INTO countries VALUES(10,'Canada','North America',0);
INSERT INTO countries VALUES(11,'Australia','Australia',0);
INSERT INTO countries VALUES(12,'Germany','Europe',0);
INSERT INTO countries VALUES(13,'Kenya','Africa',0);
INSERT INTO countries VALUES(14,'Mexico','North America',0);
INSERT INTO countries VALUES(15,'India','Asia',0);
INSERT INTO countries VALUES(16,'New Zealand','Oceania',0);
INSERT INTO countries VALUES(17,'Barbados','North America',0);
INSERT INTO countries VALUES(18,'Senegal','Africa',0);
INSERT INTO countries VALUES(19,'Mongolia','Asia',0);
INSERT INTO countries VALUES(23,'Italy','Europe',0);
INSERT INTO countries VALUES(24,'Guinea','Africa',0);
INSERT INTO countries VALUES(28,'Malta','Europe',0);
CREATE TABLE IF NOT EXISTS "cities" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    country_id INTEGER,
    FOREIGN KEY (country_id) REFERENCES countries(id)
);
INSERT INTO cities VALUES(1,'Warsaw',1);
INSERT INTO cities VALUES(2,'Krakow',1);
INSERT INTO cities VALUES(3,'London',2);
INSERT INTO cities VALUES(4,'Manchester',2);
INSERT INTO cities VALUES(5,'Jakarta',3);
INSERT INTO cities VALUES(6,'Bali',3);
INSERT INTO cities VALUES(7,'Bangkok',4);
INSERT INTO cities VALUES(8,'Chiang Mai',4);
INSERT INTO cities VALUES(9,'Johannesburg',6);
INSERT INTO cities VALUES(10,'Cape Town',6);
INSERT INTO cities VALUES(11,'Paris',7);
INSERT INTO cities VALUES(12,'Lyon',7);
INSERT INTO cities VALUES(13,'Tokyo',8);
INSERT INTO cities VALUES(14,'Kyoto',8);
INSERT INTO cities VALUES(15,'Rio de Janeiro',9);
INSERT INTO cities VALUES(16,'Sao Paulo',9);
INSERT INTO cities VALUES(17,'Toronto',10);
INSERT INTO cities VALUES(18,'Vancouver',10);
INSERT INTO cities VALUES(19,'Sydney',11);
INSERT INTO cities VALUES(20,'Melbourne',11);
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT
);
INSERT INTO users VALUES(1,'joe@gmail.com','12345','Joe','Schmoe');
INSERT INTO users VALUES(2,'jeremy@gmail.com','12345','Jeremy','Sinister');
INSERT INTO users VALUES(3,'jenny@gmail.com','12345','Jenny','Gump');
INSERT INTO users VALUES(4,'holly@gmail.com','12345','Holly','Hills');
INSERT INTO users VALUES(5,'sam@gmail.com','12345','Sam','Smith');
INSERT INTO users VALUES(6,'joe@gmail.com','123456','Joe','Joe');
INSERT INTO users VALUES(7,'rizki','1234','rizki','rizki');
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('countries',28);
COMMIT;
