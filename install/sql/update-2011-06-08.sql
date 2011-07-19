ALTER TABLE repayment ADD COLUMN creator_fk INTEGER NOT NULL; 
ALTER TABLE repayment ADD FOREIGN KEY (creator_fk) REFERENCES "user" (user_pk);
