-- This file should undo anything in `up.sql`
DROP INDEX idx_comments_user_id;
DROP INDEX idx_comments_user_id_type;
DROP INDEX idx_comments_owner_id;
DROP INDEX idx_comments_owner_id_type;

DROP TABLE comments;