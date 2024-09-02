-- Add up migration script here
-- Создание таблицы accounts, если она еще не существует
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL
);

-- Создание таблицы account_event_types, если она еще не существует
CREATE TABLE IF NOT EXISTS account_event_types (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

-- Создание таблицы account_events, если она еще не существует
CREATE TABLE IF NOT EXISTS account_events (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    event_type_id TEXT NOT NULL REFERENCES account_event_types(id),
    event_data JSONB NOT NULL
);

-- Создание индекса на поле account_id, если он еще не существует
CREATE INDEX IF NOT EXISTS idx_account_events_account_id ON account_events(account_id);

-- Создание таблицы account_emails, если она еще не существует
CREATE TABLE IF NOT EXISTS account_emails (
    email TEXT PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);