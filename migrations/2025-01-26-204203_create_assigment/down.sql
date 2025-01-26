-- This file should undo anything in `up.sql`

DROP INDEX idx_assignments_lesson_id ON assignments;
DROP INDEX idx_assignments_title ON assignments;

DROP TABLE IF EXISTS assignments RESTRICT;