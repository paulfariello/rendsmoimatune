CREATE TABLE Bdf_Security
(
  "id" SERIAL PRIMARY KEY,
  "nom" VARCHAR(10),
  "parent_fk" integer DEFAULT 1 REFERENCES Bdf_Security ON DELETE SET DEFAULT
);

CREATE TABLE Bdf_User
(
  "id" SERIAL PRIMARY KEY,
  "email" varchar(255) UNIQUE,
  "password" varchar(255)
);

CREATE TABLE Bdf_User_Right
(
  "user_cpfk" integer REFERENCES Bdf_User ON DELETE CASCADE,
  "right_cpfk" integer REFERENCES Bdf_Security ON DELETE CASCADE,
  PRIMARY KEY ("user_cpfk","right_cpfk")
);

INSERT INTO Bdf_Security ("id","nom","parent_fk") VALUES (nextval('bdf_security_id_seq'),'all',null);
