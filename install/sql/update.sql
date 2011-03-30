ALTER TABLE event RENAME TO account;
ALTER TABLE event_user RENAME TO account_user;
ALTER TABLE account RENAME COLUMN event_pk TO account_pk;
ALTER TABLE account_user RENAME COLUMN event_pcfk TO account_pcfk;
ALTER TABLE expenditure RENAME COLUMN event_fk TO account_fk;
ALTER TABLE repayment RENAME COLUMN event_fk TO account_fk;
