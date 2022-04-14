BEGIN;

CREATE TABLE account (
    _id integer NOT NULL,
    uid uuid NOT NULL,
    name character varying(255) NOT NULL
);

CREATE TABLE debt (
    _id integer NOT NULL,
    debtor_id integer NOT NULL,
    expenditure_id integer NOT NULL,
    share integer NOT NULL
);

CREATE TABLE expenditure (
    _id integer NOT NULL,
    account_id integer NOT NULL,
    name character varying(255) NOT NULL,
    date date NOT NULL,
    amount integer NOT NULL,
    payer_id integer NOT NULL
);

CREATE TABLE repayment (
    _id integer NOT NULL,
    account_id integer NOT NULL,
    date date NOT NULL,
    amount integer NOT NULL,
    payer_id integer NOT NULL,
    beneficiary_id integer NOT NULL
);

CREATE TABLE "user" (
    _id integer NOT NULL,
    account_id integer NOT NULL,
    name character varying(255) NOT NULL
);

COPY account (_id, uid, name) FROM stdin;
\.


--
-- Data for Name: debt; Type: TABLE DATA; Schema: public; Owner: rmmt
--

COPY debt (_id, debtor_id, expenditure_id, share) FROM stdin;
\.


--
-- Data for Name: expenditure; Type: TABLE DATA; Schema: public; Owner: rmmt
--

COPY expenditure (_id, account_id, name, date, amount, payer_id) FROM stdin;
\.


--
-- Data for Name: repayment; Type: TABLE DATA; Schema: public; Owner: rmmt
--

COPY repayment (_id, account_id, date, amount, payer_id, beneficiary_id) FROM stdin;
\.


--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: rmmt
--

COPY "user" (_id, account_id, name) FROM stdin;
\.

-- Add temporary original_id columns in new tables
ALTER TABLE accounts ADD COLUMN original_id INTEGER;
ALTER TABLE users ADD COLUMN original_id INTEGER;
ALTER TABLE expenditures ADD COLUMN original_id INTEGER;

-- Truncate tables
TRUNCATE accounts CASCADE;

-- Migrate from old tables
INSERT INTO accounts (original_id, id, name) SELECT _id, uid, name FROM account;
INSERT INTO users (original_id, account_id, name) SELECT u._id, a.id, u.name FROM "user" as u JOIN accounts as a ON u.account_id = a.original_id;
INSERT INTO repayments (account_id, date, amount, payer_id, beneficiary_id) SELECT a.id, r.date, r.amount, p.id, b.id FROM repayment as r JOIN accounts as a ON r.account_id = a.original_id JOIN users as p ON p.original_id = r.payer_id JOIN users as b ON b.original_id = r.beneficiary_id;
INSERT INTO expenditures (original_id, account_id, name, date, amount, payer_id) SELECT e._id, a.id, e.name, e.date, e.amount, p.id FROM expenditure as e JOIN accounts as a ON e.account_id = a.original_id JOIN users as p ON p.original_id = e.payer_id;

INSERT INTO debts (debtor_id, expenditure_id, share) SELECT u.id, e.id, d.share FROM debt as d JOIN users as u ON d.debtor_id = u.original_id JOIN expenditures as e ON e.original_id = d.expenditure_id;

-- Remove original_id columns
ALTER TABLE accounts DROP COLUMN original_id;
ALTER TABLE users DROP COLUMN original_id;
ALTER TABLE expenditures DROP COLUMN original_id;

-- Remove original tables
DROP TABLE account CASCADE;
DROP TABLE "user" CASCADE;
DROP TABLE repayment CASCADE;
DROP TABLE expenditure CASCADE;
DROP TABLE debt CASCADE;

COMMIT;
