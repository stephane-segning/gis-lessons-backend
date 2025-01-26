-- This file should undo anything in `up.sql`

DROP INDEX idx_submissions_assignment_id_status;
DROP INDEX idx_submissions_assignment_id;
DROP INDEX idx_submissions_status;

DROP TABLE IF EXISTS submissions RESTRICT;