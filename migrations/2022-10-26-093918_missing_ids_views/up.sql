-- Your SQL goes here
CREATE VIEW MISSING_REGISTRATION_IDS AS (select rid from registration_data right join (select distinct unnest(registrations) as rid from meeting_data) md on rid = registration_data.id where registration_data.id is null);
CREATE VIEW MISSING_VOTE_IDS AS (select vid from vote_data right join (select distinct unnest(voting) as vid from agenda_item) md on vid = vote_data.id where vote_data.id is null);
CREATE VIEW MISSING_MEETING_IDS AS (select meetings.id as mid from meetings left join meeting_data on meetings.id = meeting_data.id where meeting_data.id is null);