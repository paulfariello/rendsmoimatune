ALTER TABLE merge_request ADD COLUMN account_fk INTEGER NOT NULL;
ALTER TABLE merge_request ADD CONSTRAINT merge_request_account_fk_fkey FOREIGN KEY (account_fk) REFERENCES account (account_pk) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE merge_request DROP CONSTRAINT merge_request_first_user_fk_key;
ALTER TABLE merge_request ADD CONSTRAINT merge_request_first_user_fk_key UNIQUE(first_user_fk, second_user_fk);
