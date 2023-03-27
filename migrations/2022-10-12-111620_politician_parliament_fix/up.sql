-- Your SQL goes here
ALTER TABLE politician DROP CONSTRAINT politician_pkey,
    ADD CONSTRAINT politician_pkey PRIMARY KEY ("id", "parliament");