--
-- PostgreSQL database dump
--

SET statement_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = off;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET escape_string_warning = off;

SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: account; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE account (
    account_pk integer NOT NULL,
    name character varying,
    start_date date,
    end_date date,
    creator_fk integer NOT NULL,
    creation_date date
);


ALTER TABLE public.account OWNER TO rendsmoimatune_dev;

--
-- Name: account_user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE account_user (
    user_pcfk integer NOT NULL,
    account_pcfk integer NOT NULL
);


ALTER TABLE public.account_user OWNER TO rendsmoimatune_dev;

--
-- Name: beneficiary; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE beneficiary (
    beneficiary_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.beneficiary OWNER TO rendsmoimatune_dev;

--
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE beneficiary_beneficiary_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.beneficiary_beneficiary_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE beneficiary_beneficiary_pk_seq OWNED BY beneficiary.beneficiary_pk;


--
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('beneficiary_beneficiary_pk_seq', 81, true);


--
-- Name: event_event_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE event_event_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.event_event_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: event_event_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE event_event_pk_seq OWNED BY account.account_pk;


--
-- Name: event_event_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('event_event_pk_seq', 6, true);


--
-- Name: expenditure; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure (
    expenditure_pk integer NOT NULL,
    account_fk integer NOT NULL,
    title character varying,
    date date,
    amount numeric(10,2) NOT NULL,
    creator_fk integer NOT NULL
);


ALTER TABLE public.expenditure OWNER TO rendsmoimatune_dev;

--
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE expenditure_expenditure_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.expenditure_expenditure_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE expenditure_expenditure_pk_seq OWNED BY expenditure.expenditure_pk;


--
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('expenditure_expenditure_pk_seq', 47, true);


--
-- Name: expenditure_tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure_tag (
    expenditure_pcfk integer NOT NULL,
    tag_pcfk integer NOT NULL
);


ALTER TABLE public.expenditure_tag OWNER TO rendsmoimatune_dev;

--
-- Name: payer; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE payer (
    payer_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.payer OWNER TO rendsmoimatune_dev;

--
-- Name: payer_payer_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE payer_payer_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.payer_payer_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: payer_payer_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE payer_payer_pk_seq OWNED BY payer.payer_pk;


--
-- Name: payer_payer_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('payer_payer_pk_seq', 52, true);


--
-- Name: repayment; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE repayment (
    repayment_pk integer NOT NULL,
    account_fk integer NOT NULL,
    payer_fk integer NOT NULL,
    beneficiary_fk integer NOT NULL,
    date date,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.repayment OWNER TO rendsmoimatune_dev;

--
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE repayment_repayment_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.repayment_repayment_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE repayment_repayment_pk_seq OWNED BY repayment.repayment_pk;


--
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('repayment_repayment_pk_seq', 1, false);


--
-- Name: tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE tag (
    tag_pk integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.tag OWNER TO rendsmoimatune_dev;

--
-- Name: tag_tag_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE tag_tag_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.tag_tag_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: tag_tag_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE tag_tag_pk_seq OWNED BY tag.tag_pk;


--
-- Name: tag_tag_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('tag_tag_pk_seq', 1, false);


--
-- Name: user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE "user" (
    user_pk integer NOT NULL,
    email character varying NOT NULL,
    password character varying,
    name character varying,
    is_admin boolean,
    registered boolean,
    creator_fk integer,
    facebook_id integer,
    invited boolean DEFAULT false NOT NULL,
    invitation_token character varying
);


ALTER TABLE public."user" OWNER TO rendsmoimatune_dev;

--
-- Name: user_user_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE user_user_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.user_user_pk_seq OWNER TO rendsmoimatune_dev;

--
-- Name: user_user_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE user_user_pk_seq OWNED BY "user".user_pk;


--
-- Name: user_user_pk_seq; Type: SEQUENCE SET; Schema: public; Owner: rendsmoimatune
--

SELECT pg_catalog.setval('user_user_pk_seq', 27, true);


--
-- Name: account_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE account ALTER COLUMN account_pk SET DEFAULT nextval('event_event_pk_seq'::regclass);


--
-- Name: beneficiary_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE beneficiary ALTER COLUMN beneficiary_pk SET DEFAULT nextval('beneficiary_beneficiary_pk_seq'::regclass);


--
-- Name: expenditure_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE expenditure ALTER COLUMN expenditure_pk SET DEFAULT nextval('expenditure_expenditure_pk_seq'::regclass);


--
-- Name: payer_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE payer ALTER COLUMN payer_pk SET DEFAULT nextval('payer_payer_pk_seq'::regclass);


--
-- Name: repayment_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE repayment ALTER COLUMN repayment_pk SET DEFAULT nextval('repayment_repayment_pk_seq'::regclass);


--
-- Name: tag_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE tag ALTER COLUMN tag_pk SET DEFAULT nextval('tag_tag_pk_seq'::regclass);


--
-- Name: user_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE "user" ALTER COLUMN user_pk SET DEFAULT nextval('user_user_pk_seq'::regclass);


--
-- Data for Name: account; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY account (account_pk, name, start_date, end_date, creator_fk, creation_date) FROM stdin;
1	Coloc	2011-02-14	2011-07-24	1	\N
3	téléphone fix	2012-03-03	2012-04-03	16	2011-03-17
5	courses 	2012-07-11	2012-07-11	21	2011-03-19
6	test	2011-03-04	2011-03-04	21	2011-04-03
\.


--
-- Data for Name: account_user; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY account_user (user_pcfk, account_pcfk) FROM stdin;
1	1
2	1
3	1
12	1
15	1
16	3
17	3
21	5
1	5
22	5
21	6
1	6
\.


--
-- Data for Name: beneficiary; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY beneficiary (beneficiary_pk, expenditure_pcfk, user_pcfk, amount) FROM stdin;
5	3	1	19.41
6	3	2	19.41
23	14	1	20.00
24	14	2	20.00
21	12	2	16.63
1	1	1	14.80
4	2	2	2.24
3	2	1	2.25
2	1	2	14.79
25	12	1	16.62
36	23	2	27.50
37	23	1	27.50
38	24	1	7.70
39	24	2	7.70
40	25	1	55.00
41	25	2	55.00
42	26	2	9.76
43	26	1	9.76
44	27	2	3.76
45	28	2	6.46
46	28	1	6.46
47	29	16	21.00
48	29	17	21.00
51	32	21	23.30
52	32	1	23.30
53	33	1	21.00
54	33	21	21.00
55	34	1	2.00
56	34	2	2.00
57	35	1	3.85
58	35	2	3.85
59	36	1	8.80
60	36	2	8.80
61	37	21	18.30
62	37	1	18.30
63	38	21	10.00
64	38	1	10.00
65	39	1	45.50
67	39	21	45.50
68	40	1	1235.00
69	41	1	15.95
70	41	2	15.95
73	43	1	13.91
74	43	2	13.91
77	45	2	14.56
78	45	1	14.56
75	44	2	14.78
76	44	1	14.78
79	46	1	4.53
80	46	2	4.53
81	47	1	8.50
\.


--
-- Data for Name: expenditure; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY expenditure (expenditure_pk, account_fk, title, date, amount, creator_fk) FROM stdin;
14	1	Course	2011-02-26	40.00	1
2	1	Course	2011-02-17	4.49	1
3	1	Course	2011-02-21	38.82	1
12	1	Course	2011-02-24	33.25	1
23	1	Courses	2011-03-05	55.00	1
26	1	Courses	2011-03-15	19.52	1
24	1	Courses	2011-03-08	15.40	1
27	1	Ticket transilien	2011-03-10	3.76	1
25	1	Camion	2011-02-19	110.00	1
28	1	Viande	2011-03-15	12.91	1
29	3	achat téléphone	2011-03-17	42.00	1
32	5	courses	2011-03-19	46.60	1
33	5	Téléphone Fix	2011-03-16	42.00	1
34	1	Gulden Draak	2011-03-21	4.00	1
35	1	Courses	2011-03-23	7.69	1
36	1	Courses	2011-03-23	17.60	1
37	5	billet sncf	2011-03-27	36.60	1
38	5	cadeau Anna 1 an	2011-03-29	20.00	1
39	5	Train futuroscope	2011-06-04	91.00	1
1	1	Course	2011-02-15	29.59	1
40	6	test de création	2011-04-03	1235.00	21
41	1	SFR	2011-03-16	31.90	1
43	1	Courses	2011-04-07	27.81	1
45	1	Courses	2011-04-05	29.11	2
44	1	Courses	2011-04-11	29.56	2
46	1	Courses	2011-03-29	9.06	2
47	1	Pizza	2011-04-12	8.50	2
\.


--
-- Data for Name: expenditure_tag; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY expenditure_tag (expenditure_pcfk, tag_pcfk) FROM stdin;
\.


--
-- Data for Name: payer; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY payer (payer_pk, expenditure_pcfk, user_pcfk, amount) FROM stdin;
1	1	1	29.59
2	2	1	4.49
3	3	1	38.82
14	12	1	33.25
16	14	2	40.00
25	23	2	55.00
26	24	1	15.40
29	26	2	19.52
30	27	1	3.76
31	25	1	110.00
32	28	2	12.91
33	29	16	0.00
34	29	17	42.00
37	32	21	46.60
38	33	1	42.00
39	34	1	4.00
40	35	1	7.69
41	36	1	17.60
42	37	21	36.60
43	38	21	20.00
44	39	1	91.00
45	40	1	1235.00
46	41	1	31.90
48	43	1	27.81
50	45	2	29.11
49	44	2	29.56
51	46	12	9.06
52	47	2	8.50
\.


--
-- Data for Name: repayment; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY repayment (repayment_pk, account_fk, payer_fk, beneficiary_fk, date, amount) FROM stdin;
\.


--
-- Data for Name: tag; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY tag (tag_pk, name) FROM stdin;
\.


--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: rendsmoimatune
--

COPY "user" (user_pk, email, password, name, is_admin, registered, creator_fk, facebook_id, invited, invitation_token) FROM stdin;
1	4d63c51735d2b@rendsmoimatune.eu	\N	Paul Fariello	f	t	\N	1230992969	f	\N
20	paul.fariello@gmail.com	{sha512}f617a9f42cbe468519a2c37642763c9b26e7560114a7ac6cc3b6b774b909000e1f28a3cad570d26320f1793a45ad015d56df90250ebada42d8b5ca564908d02015990981744d8319066229b	bob l'éponge	f	t	1	\N	f	\N
21	4d84ca1b26004@rendsmoimatune.eu	\N	Emilie Bavoil	f	t	\N	1116783335	f	\N
2	rodiere.jean@gmail.com	{sha512}905629421bd17ae73583eed422196a23c83697414e22f96209d2fd8155d0b98c26bf65daaa4b7ca56ed80809b78843fc6f29957408b6682155402e7601711c211454802714d87a1d0f0ad2	Jean Rodière	f	t	1	\N	f	\N
3	paul.fariello@gmail.com	\N	Louis Rodière	f	\N	1	\N	t	4d934b05d0d51
12	lanfeust.de.troyes@gmail.com	\N	Émilie Bavoil	f	\N	1	\N	t	4d87a19d471c9
15	4d76a8eb17d04@rendsmoimatune.eu	\N	Elisabeth Sainton	f	\N	1	\N	f	\N
17	4d827a465d1e7@rendsmoimatune.eu	\N	fariello paul	f	\N	16	\N	f	\N
16	bavoilemilie@gmail.com	{sha512}88768b0254ab2d6eafc82b1ec5019306fbc5d89790f0472578b863d69df2a8292f8bc000d2f8a6508b6aa53c5e1dbd278d29835beb446a25b440d037f123f58a14143082324d82799151844	BAVOIL Emilie	f	t	\N	\N	f	\N
22	4d9220328c4ba@rendsmoimatune.eu	\N	bavoil	f	\N	21	\N	f	\N
27	4d9ea28c1685f@rendsmoimatune.eu	\N	Jean Rodière	f	t	\N	1025779179	f	\N
\.


--
-- Name: event_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY account
    ADD CONSTRAINT event_pkey PRIMARY KEY (account_pk);


--
-- Name: event_user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY account_user
    ADD CONSTRAINT event_user_pkey PRIMARY KEY (user_pcfk, account_pcfk);


--
-- Name: expenditure_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_pkey PRIMARY KEY (expenditure_pk);


--
-- Name: expenditure_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_pkey PRIMARY KEY (expenditure_pcfk, tag_pcfk);


--
-- Name: involved_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- Name: involved_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_pkey PRIMARY KEY (beneficiary_pk);


--
-- Name: payed_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- Name: payed_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_pkey PRIMARY KEY (payer_pk);


--
-- Name: repayment_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_pkey PRIMARY KEY (repayment_pk);


--
-- Name: tag_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (tag_pk);


--
-- Name: user_email_registered_unique; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_email_registered_unique UNIQUE (email, registered);


--
-- Name: user_facebook_id_key; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_facebook_id_key UNIQUE (facebook_id);


--
-- Name: user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (user_pk);


--
-- Name: fki_; Type: INDEX; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE INDEX fki_ ON payer USING btree (user_pcfk);


--
-- Name: fki_creator; Type: INDEX; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE INDEX fki_creator ON "user" USING btree (creator_fk);


--
-- Name: event_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY account
    ADD CONSTRAINT event_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;


--
-- Name: event_user_event_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY account_user
    ADD CONSTRAINT event_user_event_pcfk_fkey FOREIGN KEY (account_pcfk) REFERENCES account(account_pk) ON DELETE CASCADE;


--
-- Name: event_user_user_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY account_user
    ADD CONSTRAINT event_user_user_pcfk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: expenditure_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk);


--
-- Name: expenditure_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_event_fk_fkey FOREIGN KEY (account_fk) REFERENCES account(account_pk) ON DELETE CASCADE;


--
-- Name: expenditure_tag_expenditure_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_expenditure_pcfk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- Name: expenditure_tag_tag_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_tag_pcfk_fkey FOREIGN KEY (tag_pcfk) REFERENCES tag(tag_pk) ON DELETE CASCADE;


--
-- Name: involved_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- Name: involved_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: payed_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- Name: payed_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: repayment_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_event_fk_fkey FOREIGN KEY (account_fk) REFERENCES account(account_pk) ON DELETE CASCADE;


--
-- Name: repayment_from_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_from_user_fk_fkey FOREIGN KEY (payer_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: repayment_to_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_to_user_fk_fkey FOREIGN KEY (beneficiary_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: user_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE SET NULL;


--
-- Name: public; Type: ACL; Schema: -; Owner: pgsql
--

REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM pgsql;
GRANT ALL ON SCHEMA public TO pgsql;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

