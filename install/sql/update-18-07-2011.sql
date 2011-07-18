ALTER TABLE "user" DROP CONSTRAINT user_name_key;
ALTER TABLE merge_request DROP CONSTRAINT merge_request_first_user_fk_fkey;
ALTER TABLE merge_request ADD CONSTRAINT merge_request_first_user_fk_fkey FOREIGN KEY (first_user_fk) REFERENCES "user" (user_pk) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE CASCADE;
ALTER TABLE merge_request DROP CONSTRAINT merge_request_second_user_fk_fkey;
ALTER TABLE merge_request ADD CONSTRAINT merge_request_second_user_fk_fkey FOREIGN KEY (second_user_fk) REFERENCES "user" (user_pk) MATCH SIMPLE ON UPDATE NO ACTION ON DELETE CASCADE;
