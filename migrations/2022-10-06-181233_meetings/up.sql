-- Your SQL goes here
CREATE TABLE meetings (
  "id" INT PRIMARY KEY,
  "num" INT NOT NULL,
  "meeting_type" TEXT NOT NULL,
  "from" TIMESTAMP,
  "to" TIMESTAMP,
  "session" INT NOT NULL,
  "protocol_link" TEXT,
  "stenogram_link" TEXT,
  "video_comment" TEXT,
  "video_link" TEXT
)