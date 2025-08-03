-- SPL Token Memo --
CREATE TABLE IF NOT EXISTS spl_memo AS base_events
COMMENT 'SPL Token Memo V1 & V2 events';
ALTER TABLE spl_memo
    ADD COLUMN IF NOT EXISTS memo String;

ALTER TABLE spl_memo
    ADD PROJECTION IF NOT EXISTS prj_signature (SELECT * ORDER BY signature);