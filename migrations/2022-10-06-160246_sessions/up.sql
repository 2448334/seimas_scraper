-- Your SQL goes here
CREATE TABLE sessions (
  "id" INT PRIMARY KEY,
  "num" INT NOT NULL,
  "name" TEXT NOT NULL,
  "from" DATE,
  "to" DATE,
  "parliament" INT NOT NULL
)