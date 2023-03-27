-- This file should undo anything in `up.sql`
ALTER TABLE politician DROP CONSTRAINT politician_pkey,
    ADD CONSTRAINT politician_pkey PRIMARY KEY ("id");