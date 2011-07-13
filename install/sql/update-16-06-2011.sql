CREATE TABLE merge_request (
    merge_request_pk SERIAL NOT NULL PRIMARY KEY, 
    first_user_fk INTEGER NOT NULL,
    second_user_fk INTEGER NOT NULL,
    requester_fk INTEGER NOT NULL,
    first_user_agreement BOOLEAN NOT NULL DEFAULT false,
    second_user_agreement BOOLEAN NOT NULL DEFAULT false,
    first_user_request_token CHARACTER VARYING,
    second_user_request_token CHARACTER VARYING,
    FOREIGN KEY (first_user_fk) REFERENCES "user" (user_pk),
    FOREIGN KEY (second_user_fk) REFERENCES "user" (user_pk),
    FOREIGN KEY (requester_fk) REFERENCES "user" (user_pk),
    UNIQUE (first_user_fk, second_user_fk, requester_fk)
);