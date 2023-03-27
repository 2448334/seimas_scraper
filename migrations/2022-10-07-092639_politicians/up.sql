-- Your SQL goes here
CREATE TYPE PQ_DEPARTMENT_TYPE AS ENUM ('office', 'group');
CREATE TYPE PQ_GENDER AS ENUM ('m', 'f');

CREATE TABLE office (
  "id" SERIAL PRIMARY KEY,
  "department_id" INT,
  "department_name" TEXT,
  "department_type" PQ_DEPARTMENT_TYPE,
  "duties" TEXT,
  "from" DATE,
  "to" DATE
);

CREATE TABLE politician (
  "id" INT PRIMARY KEY,
  "parliament" INT NOT NULL,
  "name" TEXT NOT NULL,
  "surname" TEXT NOT NULL,
  "gender" PQ_GENDER,
  "from" DATE,
  "to" DATE,
  "party" TEXT,
  "elected_type" TEXT,
  "biography_link" TEXT,
  "term_count" INT,
  "email" TEXT,
  "phone" TEXT[] NOT NULL,
  "website" TEXT,
  "offices" INT[] NOT NULL
);