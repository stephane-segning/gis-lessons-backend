-- This file should undo anything in `up.sql`

DROP INDEX idx_submission_members_assignment_id_enrollment_id;
DROP INDEX idx_submission_members_submission_id;
DROP INDEX idx_submission_members_enrollment_id;
DROP INDEX idx_submission_members_assignment_id;

DROP TABLE IF EXISTS submission_members RESTRICT;