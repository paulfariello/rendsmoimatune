ALTER TABLE "user" ADD COLUMN connection_counter integer;
ALTER TABLE "user" ALTER COLUMN connection_counter SET NOT NULL;
ALTER TABLE "user" ALTER COLUMN connection_counter SET DEFAULT 0;
