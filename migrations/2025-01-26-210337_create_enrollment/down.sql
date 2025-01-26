-- This file should undo anything in `up.sql`
DROP INDEX idx_enrollments_user_id;
DROP INDEX idx_enrollments_user_id_enrollment_type;
DROP INDEX idx_enrollments_course_id;
DROP INDEX idx_enrollments_course_id_enrollment_type;

DROP TABLE enrollments;