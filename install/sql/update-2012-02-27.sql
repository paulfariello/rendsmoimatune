CREATE TABLE "api_client" (
    "client_pk" SERIAL PRIMARY KEY,
    "email" character varying,
    "token" character varying,
    "api_key" character varying UNIQUE
);
ALTER TABLE "user" ADD COLUMN api_auth_token character varying DEFAULT null;
ALTER TABLE "user" ADD COLUMN api_client_fk INTEGER;
ALTER TABLE "user" ADD FOREIGN KEY (api_client_fk) REFERENCES api_client (client_pk);
