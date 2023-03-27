-- Your SQL goes here
CREATE TYPE PQ_VOTE_TYPE AS ENUM ('for', 'against', 'abstain');

CREATE TABLE vote_data (
  "id" INT,
  "person_id" INT,
  "vote" PQ_VOTE_TYPE,
  PRIMARY KEY ("id", "person_id")
);