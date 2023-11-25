PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE cities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
CREATE TABLE countries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    continent TEXT
);
INSERT INTO countries VALUES(1,'Poland','Europe');
INSERT INTO countries VALUES(2,'United Kingdom','Europe');
INSERT INTO countries VALUES(3,'Indonesia','Asia');
INSERT INTO countries VALUES(4,'Thailand','Asia');
INSERT INTO countries VALUES(5,'Botswana','Africa');
INSERT INTO countries VALUES(6,'South Africa','Africa');
INSERT INTO countries VALUES(7,'Angola',NULL);
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('countries',7);
COMMIT;
