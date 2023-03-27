-- Your SQL goes here
CREATE OR REPLACE FUNCTION GETSESSIONS(parliament integer) RETURNS TABLE (session_id integer) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY SELECT sessions.id as session_id FROM sessions WHERE sessions.parliament = parliament;
    END;
$func$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION GETAGENDAS(session_id integer) RETURNS TABLE (agenda_id integer) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY (
            SELECT UNNEST(meeting_data.agenda) AS agenda_id
                FROM meeting_data, (
                    SELECT meetings.id AS meetings_id
                    FROM meetings
                    WHERE meetings."session" = session_id) meetings_ids
                WHERE meeting_data.id = meetings_ids.meetings_id);
    END;
$func$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION GETVOTINGS(agenda_id integer) RETURNS TABLE (vote_id integer) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY (
            SELECT UNNEST(agenda_item.voting) AS vote_id
                FROM agenda_item
                WHERE agenda_item.id = agenda_id);
    END;
$func$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION GETVOTINGS_BYPARLIAMENT(parliament integer) RETURNS TABLE (vote_id integer) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY SELECT GETVOTINGS(agenda_id) as vote_id FROM (SELECT GETAGENDAS(session_id) as agenda_id FROM GETSESSIONS(parliament)) agenda;
    END;
$func$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION GETVOTINGS_BYPARLIAMENT(parliament integer) RETURNS TABLE (vote_id integer) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY SELECT GETVOTINGS(agenda_id) as vote_id FROM (SELECT GETAGENDAS(session_id) as agenda_id FROM GETSESSIONS(parliament)) agenda;
    END;
$func$ LANGUAGE plpgsql;


CREATE OR REPLACE FUNCTION GETVOTES_BYPARLIAMENT(parliament integer) RETURNS TABLE (
    "person_id" integer,
    "name" text,
    "surname" text,
    "party" text,
    "for_count" bigint,
    "against_count" bigint,
    "abstain_count" bigint,
    "none_count" bigint,
    "voted_count" bigint,
    "all_count" bigint,
    "for_percent" numeric,
    "against_percent" numeric,
    "abstain_percent" numeric,
    "voted_percent" numeric) AS $func$
    #variable_conflict use_variable
    BEGIN
        RETURN QUERY (SELECT 
            "_person_id" as "person_id",
            "_name" as "name",
            "_surname" as "surname",
            "_party" as "party",
            "_for_count" as "for_count",
            "_against_count" as "against_count",
            "_abstain_count" as "abstain_count",
            "_none_count" as "none_count",
            "_voted_count" as "voted_count",
            "_all_count" as "all_count",
            ROUND(CAST("_for_count" as DECIMAL)/GREATEST("_voted_count", 1) * 100, 2) as "for_percent",
            ROUND(CAST("_against_count" as DECIMAL)/GREATEST("_voted_count", 1) * 100, 2) as "against_percent",
            ROUND(CAST("_abstain_count" as DECIMAL)/GREATEST("_voted_count", 1) * 100, 2) as "abstain_percent",
            ROUND(CAST("_voted_count" as DECIMAL)/GREATEST("_all_count", 1) * 100, 2) as "voted_percent"
        FROM (
            SELECT
            "politician"."id" as "_person_id",
            "politician"."name" as "_name",
            "politician"."surname" as "_surname",
            "politician"."party" as "_party",
            COUNT(*) FILTER(WHERE vote = 'for') as "_for_count",
            COUNT(*) FILTER(WHERE vote = 'against') as "_against_count",
            COUNT(*) FILTER(WHERE vote = 'abstain') as "_abstain_count",
            COUNT(*) FILTER(WHERE vote IS NULL) as "_none_count",
            COUNT(*) FILTER(WHERE vote IS NOT NULL) as "_voted_count",
            COUNT(*) as "_all_count"
        FROM (
            SELECT 
                "vote_data"."id",
                "vote_data"."person_id",
                "vote_data"."vote"
            FROM 
                vote_data,
                GETVOTINGS_BYPARLIAMENT(parliament) recent_votes
            WHERE vote_data.id = recent_votes
            ) AS "recent_vote_data"
        INNER JOIN politician ON recent_vote_data.person_id = politician.id
        WHERE parliament = 9 AND politician."to" IS NULL
        GROUP BY politician.id, politician.parliament) 
        AS "subqueries"
        ORDER BY "surname");

    END;
$func$ LANGUAGE plpgsql;

