-- This file should undo anything in `up.sql`
DROP INDEX idx_accounts_sub ON accounts;
DROP INDEX idx_accounts_name ON accounts;

DROP TABLE accounts;