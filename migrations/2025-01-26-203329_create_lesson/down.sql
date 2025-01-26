-- This file should undo anything in `up.sql`

DROP INDEX idx_lessons_module_id ON lessons;
DROP INDEX idx_lessons_title ON lessons;

DROP TABLE IF EXISTS lessons RESTRICT;