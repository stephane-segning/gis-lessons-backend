-- This file should undo anything in `up.sql`

DROP INDEX idx_modules_course_id ON modules;
DROP INDEX idx_modules_title ON modules;

DROP TABLE modules;