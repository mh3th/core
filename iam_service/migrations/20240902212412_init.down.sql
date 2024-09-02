-- Add down migration script here
-- Удаление таблицы account_emails, если она существует
DROP TABLE IF EXISTS account_emails;

-- Удаление индекса и таблицы account_events, если они существуют
DROP INDEX IF EXISTS idx_account_events_account_id;
DROP TABLE IF EXISTS account_events;

-- Удаление таблицы account_event_types, если она существует
DROP TABLE IF EXISTS account_event_types;

-- Удаление таблицы accounts, если она существует
DROP TABLE IF EXISTS accounts;
