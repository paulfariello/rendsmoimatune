--
-- PostgreSQL database dump
--

-- Started on 2011-01-01 17:38:32 CET

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
-- TOC entry 1518 (class 1259 OID 25506)
-- Dependencies: 6
-- Name: beneficiary; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE beneficiary (
    beneficiary_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1519 (class 1259 OID 25509)
-- Dependencies: 1518 6
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE beneficiary_beneficiary_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1862 (class 0 OID 0)
-- Dependencies: 1519
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE beneficiary_beneficiary_pk_seq OWNED BY beneficiary.beneficiary_pk;


--
-- TOC entry 1520 (class 1259 OID 25511)
-- Dependencies: 6
-- Name: event; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE event (
    event_pk integer NOT NULL,
    name character varying,
    start_date date,
    end_date date
);


--
-- TOC entry 1521 (class 1259 OID 25517)
-- Dependencies: 1520 6
-- Name: event_event_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE event_event_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1863 (class 0 OID 0)
-- Dependencies: 1521
-- Name: event_event_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE event_event_pk_seq OWNED BY event.event_pk;


--
-- TOC entry 1522 (class 1259 OID 25519)
-- Dependencies: 6
-- Name: event_user; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE event_user (
    user_pcfk integer NOT NULL,
    event_pcfk integer NOT NULL
);


--
-- TOC entry 1523 (class 1259 OID 25522)
-- Dependencies: 6
-- Name: expenditure; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE expenditure (
    expenditure_pk integer NOT NULL,
    event_fk integer NOT NULL,
    title character varying,
    date date,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1524 (class 1259 OID 25528)
-- Dependencies: 6 1523
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE expenditure_expenditure_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1864 (class 0 OID 0)
-- Dependencies: 1524
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE expenditure_expenditure_pk_seq OWNED BY expenditure.expenditure_pk;


--
-- TOC entry 1525 (class 1259 OID 25530)
-- Dependencies: 6
-- Name: expenditure_tag; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE expenditure_tag (
    expenditure_pcfk integer NOT NULL,
    tag_pcfk integer NOT NULL
);


--
-- TOC entry 1526 (class 1259 OID 25533)
-- Dependencies: 6
-- Name: payer; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE payer (
    payer_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1527 (class 1259 OID 25536)
-- Dependencies: 1526 6
-- Name: payer_payer_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE payer_payer_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1865 (class 0 OID 0)
-- Dependencies: 1527
-- Name: payer_payer_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE payer_payer_pk_seq OWNED BY payer.payer_pk;


--
-- TOC entry 1528 (class 1259 OID 25538)
-- Dependencies: 6
-- Name: repayment; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE repayment (
    repayment_pk integer NOT NULL,
    event_fk integer NOT NULL,
    payer_fk integer NOT NULL,
    beneficiary_fk integer NOT NULL,
    date date,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1529 (class 1259 OID 25541)
-- Dependencies: 1528 6
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE repayment_repayment_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1866 (class 0 OID 0)
-- Dependencies: 1529
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE repayment_repayment_pk_seq OWNED BY repayment.repayment_pk;


--
-- TOC entry 1530 (class 1259 OID 25543)
-- Dependencies: 6
-- Name: tag; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE tag (
    tag_pk integer NOT NULL,
    name character varying NOT NULL
);


--
-- TOC entry 1531 (class 1259 OID 25549)
-- Dependencies: 1530 6
-- Name: tag_tag_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE tag_tag_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1867 (class 0 OID 0)
-- Dependencies: 1531
-- Name: tag_tag_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE tag_tag_pk_seq OWNED BY tag.tag_pk;


--
-- TOC entry 1532 (class 1259 OID 25551)
-- Dependencies: 1817 6
-- Name: user; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE "user" (
    user_pk integer NOT NULL,
    email character varying NOT NULL,
    password character varying,
    first_name character varying,
    last_name character varying,
    is_admin boolean,
    registered boolean DEFAULT true,
    creator_fk integer,
    facebook_id integer
);


--
-- TOC entry 1533 (class 1259 OID 25558)
-- Dependencies: 1532 6
-- Name: user_user_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE user_user_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1868 (class 0 OID 0)
-- Dependencies: 1533
-- Name: user_user_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE user_user_pk_seq OWNED BY "user".user_pk;


--
-- TOC entry 1811 (class 2604 OID 25560)
-- Dependencies: 1519 1518
-- Name: beneficiary_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE beneficiary ALTER COLUMN beneficiary_pk SET DEFAULT nextval('beneficiary_beneficiary_pk_seq'::regclass);


--
-- TOC entry 1812 (class 2604 OID 25561)
-- Dependencies: 1521 1520
-- Name: event_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE event ALTER COLUMN event_pk SET DEFAULT nextval('event_event_pk_seq'::regclass);


--
-- TOC entry 1813 (class 2604 OID 25562)
-- Dependencies: 1524 1523
-- Name: expenditure_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE expenditure ALTER COLUMN expenditure_pk SET DEFAULT nextval('expenditure_expenditure_pk_seq'::regclass);


--
-- TOC entry 1814 (class 2604 OID 25563)
-- Dependencies: 1527 1526
-- Name: payer_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE payer ALTER COLUMN payer_pk SET DEFAULT nextval('payer_payer_pk_seq'::regclass);


--
-- TOC entry 1815 (class 2604 OID 25564)
-- Dependencies: 1529 1528
-- Name: repayment_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE repayment ALTER COLUMN repayment_pk SET DEFAULT nextval('repayment_repayment_pk_seq'::regclass);


--
-- TOC entry 1816 (class 2604 OID 25565)
-- Dependencies: 1531 1530
-- Name: tag_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE tag ALTER COLUMN tag_pk SET DEFAULT nextval('tag_tag_pk_seq'::regclass);


--
-- TOC entry 1818 (class 2604 OID 25566)
-- Dependencies: 1533 1532
-- Name: user_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE "user" ALTER COLUMN user_pk SET DEFAULT nextval('user_user_pk_seq'::regclass);


--
-- TOC entry 1824 (class 2606 OID 25568)
-- Dependencies: 1520 1520
-- Name: event_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY event
    ADD CONSTRAINT event_pkey PRIMARY KEY (event_pk);


--
-- TOC entry 1826 (class 2606 OID 25570)
-- Dependencies: 1522 1522 1522
-- Name: event_user_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_pkey PRIMARY KEY (user_pcfk, event_pcfk);


--
-- TOC entry 1828 (class 2606 OID 25572)
-- Dependencies: 1523 1523
-- Name: expenditure_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_pkey PRIMARY KEY (expenditure_pk);


--
-- TOC entry 1830 (class 2606 OID 25574)
-- Dependencies: 1525 1525 1525
-- Name: expenditure_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_pkey PRIMARY KEY (expenditure_pcfk, tag_pcfk);


--
-- TOC entry 1820 (class 2606 OID 25576)
-- Dependencies: 1518 1518 1518
-- Name: involved_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- TOC entry 1822 (class 2606 OID 25578)
-- Dependencies: 1518 1518
-- Name: involved_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_pkey PRIMARY KEY (beneficiary_pk);


--
-- TOC entry 1833 (class 2606 OID 25580)
-- Dependencies: 1526 1526 1526
-- Name: payed_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- TOC entry 1835 (class 2606 OID 25582)
-- Dependencies: 1526 1526
-- Name: payed_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_pkey PRIMARY KEY (payer_pk);


--
-- TOC entry 1837 (class 2606 OID 25584)
-- Dependencies: 1528 1528
-- Name: repayment_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_pkey PRIMARY KEY (repayment_pk);


--
-- TOC entry 1839 (class 2606 OID 25586)
-- Dependencies: 1530 1530
-- Name: tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (tag_pk);


--
-- TOC entry 1842 (class 2606 OID 25671)
-- Dependencies: 1532 1532
-- Name: user_facebook_id_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_facebook_id_key UNIQUE (facebook_id);


--
-- TOC entry 1844 (class 2606 OID 25588)
-- Dependencies: 1532 1532
-- Name: user_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (user_pk);


--
-- TOC entry 1831 (class 1259 OID 25589)
-- Dependencies: 1526
-- Name: fki_; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX fki_ ON payer USING btree (user_pcfk);


--
-- TOC entry 1840 (class 1259 OID 25661)
-- Dependencies: 1532
-- Name: fki_creator; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX fki_creator ON "user" USING btree (creator_fk);


--
-- TOC entry 1847 (class 2606 OID 25590)
-- Dependencies: 1823 1520 1522
-- Name: event_user_event_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_event_pcfk_fkey FOREIGN KEY (event_pcfk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1848 (class 2606 OID 25595)
-- Dependencies: 1843 1532 1522
-- Name: event_user_user_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_user_pcfk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1849 (class 2606 OID 25600)
-- Dependencies: 1523 1823 1520
-- Name: expenditure_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1850 (class 2606 OID 25605)
-- Dependencies: 1523 1827 1525
-- Name: expenditure_tag_expenditure_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_expenditure_pcfk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1851 (class 2606 OID 25610)
-- Dependencies: 1838 1530 1525
-- Name: expenditure_tag_tag_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_tag_pcfk_fkey FOREIGN KEY (tag_pcfk) REFERENCES tag(tag_pk) ON DELETE CASCADE;


--
-- TOC entry 1845 (class 2606 OID 25615)
-- Dependencies: 1523 1518 1827
-- Name: involved_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1846 (class 2606 OID 25620)
-- Dependencies: 1518 1532 1843
-- Name: involved_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1852 (class 2606 OID 25625)
-- Dependencies: 1827 1526 1523
-- Name: payed_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1853 (class 2606 OID 25630)
-- Dependencies: 1526 1843 1532
-- Name: payed_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1854 (class 2606 OID 25635)
-- Dependencies: 1528 1823 1520
-- Name: repayment_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1855 (class 2606 OID 25640)
-- Dependencies: 1532 1843 1528
-- Name: repayment_from_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_from_user_fk_fkey FOREIGN KEY (payer_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1856 (class 2606 OID 25645)
-- Dependencies: 1843 1528 1532
-- Name: repayment_to_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_to_user_fk_fkey FOREIGN KEY (beneficiary_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1857 (class 2606 OID 25656)
-- Dependencies: 1532 1532 1843
-- Name: user_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE SET NULL;


--
-- TOC entry 1861 (class 0 OID 0)
-- Dependencies: 6
-- Name: public; Type: ACL; Schema: -; Owner: -
--

REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM postgres;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO PUBLIC;


-- Completed on 2011-01-01 17:38:33 CET

--
-- PostgreSQL database dump complete
--

