BEGIN;

ALTER TABLE api_client_client_pk_seq RENAME TO api_oauth_consumer_api_oauth_consumer_pk_seq;

ALTER TABLE api_client RENAME TO api_oauth_consumer;
ALTER TABLE api_oauth_consumer RENAME COLUMN client_pk TO api_oauth_consumer_pk;
ALTER TABLE api_oauth_consumer ALTER COLUMN api_oauth_consumer_pk SET DEFAULT nextval('api_oauth_consumer_api_oauth_consumer_pk_seq');
ALTER TABLE api_oauth_consumer RENAME COLUMN token TO registration_token;
ALTER TABLE api_oauth_consumer ALTER COLUMN registration_token TYPE character(40);
ALTER TABLE api_oauth_consumer RENAME COLUMN api_key TO key;
ALTER TABLE api_oauth_consumer ALTER COLUMN key TYPE character(40);
ALTER TABLE api_oauth_consumer ADD COLUMN secret character(40);
ALTER TABLE api_oauth_consumer ADD COLUMN name character varying;
ALTER TABLE api_oauth_consumer ADD COLUMN url character varying;

ALTER TABLE api_oauth_consumer ADD CONSTRAINT email_key UNIQUE(email);
ALTER TABLE api_oauth_consumer ADD CONSTRAINT key_key UNIQUE(key);

ALTER TABLE "user" DROP COLUMN api_auth_token;
ALTER TABLE "user" DROP COLUMN api_client_fk;

CREATE TABLE "api_oauth_token" (
    "api_oauth_token_pk" SERIAL PRIMARY KEY,
    "callback" character varying,
    "token" character(40) NOT NULL,
    "secret" character(40) NOT NULL,
    "verifier" character(40),
    "creation_date" TIMESTAMP(0) WITHOUT TIME ZONE DEFAULT now() NOT NULL,
    "duration" integer,
    "consumer_fk" integer NOT NULL,
    "user_fk" integer,
    "is_access_token" boolean NOT NULL,
    FOREIGN KEY ( consumer_fk ) REFERENCES api_oauth_consumer ( api_oauth_consumer_pk ),
    FOREIGN KEY ( user_fk ) REFERENCES "user" ( user_pk ),
    UNIQUE (consumer_fk, token),
    UNIQUE (verifier, is_access_token),
    CHECK ( user_fk IS NOT NULL OR NOT is_access_token )
);

COMMIT;
