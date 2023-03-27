-- Your SQL goes here
CREATE TABLE meeting_data (
  "id" INT PRIMARY KEY,
  "from" TIMESTAMP,
  "to" TIMESTAMP,
  "agenda" INT[] NOT NULL,
  "registrations" INT[] NOT NULL
);

CREATE TABLE agenda_item (
  "id" INT PRIMARY KEY,
  "agenda_state_id" INT,
  "agenda_group_id" INT,
  "document_key" INT,
  "nr" TEXT,
  "name" TEXT,
  "state" TEXT,
  "agenda_type" TEXT,
  "from" TIMESTAMP,
  "to" TIMESTAMP,
  "speeches" INT[] NOT NULL,
  "voting" INT[] NOT NULL
);

CREATE TABLE speech (
  "id" INT PRIMARY KEY,
  "discussion_id" INT,
  "person_id" INT,
  "person" TEXT,
  "office" TEXT,
  "from" TIMESTAMP,
  "to" TIMESTAMP
);

CREATE TABLE vote (
  "id" INT PRIMARY KEY,
  "summary" TEXT,
  "result" TEXT,
  "from" TIMESTAMP,
  "to" TIMESTAMP
);

CREATE TABLE registration (
  "id" INT PRIMARY KEY,
  "result" TEXT,
  "from" TIMESTAMP,
  "to" TIMESTAMP
);

