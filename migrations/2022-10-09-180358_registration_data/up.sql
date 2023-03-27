-- Your SQL goes here
CREATE TABLE registration_data (
  "id" INT,
  "person_id" INT,
  "registered" BOOLEAN,
  PRIMARY KEY ("id", "person_id")
);