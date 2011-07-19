ALTER TABLE merge_request DROP CONSTRAINT merge_request_requester_fk_fkey;

ALTER TABLE merge_request
  ADD CONSTRAINT merge_request_requester_fk_fkey FOREIGN KEY (requester_fk)
      REFERENCES "user" (user_pk) MATCH SIMPLE
          ON UPDATE NO ACTION ON DELETE CASCADE;

