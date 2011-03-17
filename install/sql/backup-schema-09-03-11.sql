--
-- PostgreSQL database dump
--

-- Started on 2011-03-09 18:38:13 CET

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
-- TOC entry 1531 (class 1259 OID 23168)
-- Dependencies: 3
-- Name: beneficiary; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE beneficiary (
    beneficiary_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1530 (class 1259 OID 23166)
-- Dependencies: 1531 3
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE beneficiary_beneficiary_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1864 (class 0 OID 0)
-- Dependencies: 1530
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE beneficiary_beneficiary_pk_seq OWNED BY beneficiary.beneficiary_pk;


--
-- TOC entry 1519 (class 1259 OID 23065)
-- Dependencies: 1812 3
-- Name: event; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE event (
    event_pk integer NOT NULL,
    name character varying,
    start_date date,
    end_date date,
    creation_date date DEFAULT now() NOT NULL,
    creator_fk integer
);


--
-- TOC entry 1518 (class 1259 OID 23063)
-- Dependencies: 1519 3
-- Name: event_event_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE event_event_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1865 (class 0 OID 0)
-- Dependencies: 1518
-- Name: event_event_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE event_event_pk_seq OWNED BY event.event_pk;


--
-- TOC entry 1524 (class 1259 OID 23108)
-- Dependencies: 3
-- Name: event_user; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE event_user (
    user_pcfk integer NOT NULL,
    event_pcfk integer NOT NULL
);


--
-- TOC entry 1528 (class 1259 OID 23136)
-- Dependencies: 3
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
-- TOC entry 1527 (class 1259 OID 23134)
-- Dependencies: 1528 3
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE expenditure_expenditure_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1866 (class 0 OID 0)
-- Dependencies: 1527
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE expenditure_expenditure_pk_seq OWNED BY expenditure.expenditure_pk;


--
-- TOC entry 1529 (class 1259 OID 23150)
-- Dependencies: 3
-- Name: expenditure_tag; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE expenditure_tag (
    expenditure_pcfk integer NOT NULL,
    tag_pcfk integer NOT NULL
);


--
-- TOC entry 1533 (class 1259 OID 23188)
-- Dependencies: 3
-- Name: payer; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE payer (
    payer_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


--
-- TOC entry 1532 (class 1259 OID 23186)
-- Dependencies: 1533 3
-- Name: payer_payer_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE payer_payer_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1867 (class 0 OID 0)
-- Dependencies: 1532
-- Name: payer_payer_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE payer_payer_pk_seq OWNED BY payer.payer_pk;


--
-- TOC entry 1523 (class 1259 OID 23087)
-- Dependencies: 3
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
-- TOC entry 1522 (class 1259 OID 23085)
-- Dependencies: 1523 3
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE repayment_repayment_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1868 (class 0 OID 0)
-- Dependencies: 1522
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE repayment_repayment_pk_seq OWNED BY repayment.repayment_pk;


--
-- TOC entry 1526 (class 1259 OID 23125)
-- Dependencies: 3
-- Name: tag; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE tag (
    tag_pk integer NOT NULL,
    name character varying NOT NULL
);


--
-- TOC entry 1525 (class 1259 OID 23123)
-- Dependencies: 3 1526
-- Name: tag_tag_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE tag_tag_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1869 (class 0 OID 0)
-- Dependencies: 1525
-- Name: tag_tag_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE tag_tag_pk_seq OWNED BY tag.tag_pk;


--
-- TOC entry 1521 (class 1259 OID 23076)
-- Dependencies: 1814 1815 3
-- Name: user; Type: TABLE; Schema: public; Owner: -; Tablespace: 
--

CREATE TABLE "user" (
    user_pk integer NOT NULL,
    email character varying NOT NULL,
    password character varying,
    name character varying,
    is_admin boolean,
    registered boolean DEFAULT true,
    creator_fk integer,
    facebook_id character varying,
    invited boolean DEFAULT false NOT NULL,
    invitation_token character varying
);


--
-- TOC entry 1520 (class 1259 OID 23074)
-- Dependencies: 1521 3
-- Name: user_user_pk_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE user_user_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


--
-- TOC entry 1870 (class 0 OID 0)
-- Dependencies: 1520
-- Name: user_user_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE user_user_pk_seq OWNED BY "user".user_pk;


--
-- TOC entry 1819 (class 2604 OID 31457)
-- Dependencies: 1530 1531 1531
-- Name: beneficiary_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE beneficiary ALTER COLUMN beneficiary_pk SET DEFAULT nextval('beneficiary_beneficiary_pk_seq'::regclass);


--
-- TOC entry 1811 (class 2604 OID 23068)
-- Dependencies: 1518 1519 1519
-- Name: event_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE event ALTER COLUMN event_pk SET DEFAULT nextval('event_event_pk_seq'::regclass);


--
-- TOC entry 1818 (class 2604 OID 23139)
-- Dependencies: 1528 1527 1528
-- Name: expenditure_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE expenditure ALTER COLUMN expenditure_pk SET DEFAULT nextval('expenditure_expenditure_pk_seq'::regclass);


--
-- TOC entry 1820 (class 2604 OID 31458)
-- Dependencies: 1532 1533 1533
-- Name: payer_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE payer ALTER COLUMN payer_pk SET DEFAULT nextval('payer_payer_pk_seq'::regclass);


--
-- TOC entry 1816 (class 2604 OID 23090)
-- Dependencies: 1523 1522 1523
-- Name: repayment_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE repayment ALTER COLUMN repayment_pk SET DEFAULT nextval('repayment_repayment_pk_seq'::regclass);


--
-- TOC entry 1817 (class 2604 OID 23128)
-- Dependencies: 1525 1526 1526
-- Name: tag_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE tag ALTER COLUMN tag_pk SET DEFAULT nextval('tag_tag_pk_seq'::regclass);


--
-- TOC entry 1813 (class 2604 OID 23079)
-- Dependencies: 1520 1521 1521
-- Name: user_pk; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE "user" ALTER COLUMN user_pk SET DEFAULT nextval('user_user_pk_seq'::regclass);


--
-- TOC entry 1822 (class 2606 OID 23073)
-- Dependencies: 1519 1519
-- Name: event_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY event
    ADD CONSTRAINT event_pkey PRIMARY KEY (event_pk);


--
-- TOC entry 1830 (class 2606 OID 23223)
-- Dependencies: 1524 1524 1524
-- Name: event_user_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_pkey PRIMARY KEY (user_pcfk, event_pcfk);


--
-- TOC entry 1834 (class 2606 OID 23144)
-- Dependencies: 1528 1528
-- Name: expenditure_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_pkey PRIMARY KEY (expenditure_pk);


--
-- TOC entry 1836 (class 2606 OID 23154)
-- Dependencies: 1529 1529 1529
-- Name: expenditure_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_pkey PRIMARY KEY (expenditure_pcfk, tag_pcfk);


--
-- TOC entry 1838 (class 2606 OID 23175)
-- Dependencies: 1531 1531 1531
-- Name: involved_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- TOC entry 1840 (class 2606 OID 23173)
-- Dependencies: 1531 1531
-- Name: involved_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_pkey PRIMARY KEY (beneficiary_pk);


--
-- TOC entry 1843 (class 2606 OID 23195)
-- Dependencies: 1533 1533 1533
-- Name: payed_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_key UNIQUE (expenditure_pcfk, user_pcfk);


--
-- TOC entry 1845 (class 2606 OID 23193)
-- Dependencies: 1533 1533
-- Name: payed_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_pkey PRIMARY KEY (payer_pk);


--
-- TOC entry 1828 (class 2606 OID 23092)
-- Dependencies: 1523 1523
-- Name: repayment_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_pkey PRIMARY KEY (repayment_pk);


--
-- TOC entry 1832 (class 2606 OID 23133)
-- Dependencies: 1526 1526
-- Name: tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (tag_pk);


--
-- TOC entry 1824 (class 2606 OID 39857)
-- Dependencies: 1521 1521
-- Name: user_name_key; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_name_key UNIQUE (name);


--
-- TOC entry 1826 (class 2606 OID 23084)
-- Dependencies: 1521 1521
-- Name: user_pkey; Type: CONSTRAINT; Schema: public; Owner: -; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (user_pk);


--
-- TOC entry 1841 (class 1259 OID 23216)
-- Dependencies: 1533
-- Name: fki_; Type: INDEX; Schema: public; Owner: -; Tablespace: 
--

CREATE INDEX fki_ ON payer USING btree (user_pcfk);


--
-- TOC entry 1846 (class 2606 OID 39880)
-- Dependencies: 1519 1825 1521
-- Name: event_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY event
    ADD CONSTRAINT event_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;


--
-- TOC entry 1851 (class 2606 OID 23224)
-- Dependencies: 1821 1524 1519
-- Name: event_user_event_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_event_pcfk_fkey FOREIGN KEY (event_pcfk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1852 (class 2606 OID 23229)
-- Dependencies: 1524 1521 1825
-- Name: event_user_user_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_user_pcfk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1853 (class 2606 OID 23234)
-- Dependencies: 1519 1528 1821
-- Name: expenditure_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1854 (class 2606 OID 23239)
-- Dependencies: 1529 1833 1528
-- Name: expenditure_tag_expenditure_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_expenditure_pcfk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1855 (class 2606 OID 23244)
-- Dependencies: 1526 1529 1831
-- Name: expenditure_tag_tag_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_tag_pcfk_fkey FOREIGN KEY (tag_pcfk) REFERENCES tag(tag_pk) ON DELETE CASCADE;


--
-- TOC entry 1856 (class 2606 OID 23176)
-- Dependencies: 1531 1833 1528
-- Name: involved_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1857 (class 2606 OID 23181)
-- Dependencies: 1531 1825 1521
-- Name: involved_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY beneficiary
    ADD CONSTRAINT involved_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1858 (class 2606 OID 23206)
-- Dependencies: 1833 1533 1528
-- Name: payed_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_expenditure_fk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1859 (class 2606 OID 23211)
-- Dependencies: 1521 1533 1825
-- Name: payed_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY payer
    ADD CONSTRAINT payed_user_fk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1848 (class 2606 OID 23249)
-- Dependencies: 1821 1519 1523
-- Name: repayment_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1849 (class 2606 OID 23254)
-- Dependencies: 1521 1523 1825
-- Name: repayment_from_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_from_user_fk_fkey FOREIGN KEY (payer_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1850 (class 2606 OID 23259)
-- Dependencies: 1521 1825 1523
-- Name: repayment_to_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_to_user_fk_fkey FOREIGN KEY (beneficiary_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1847 (class 2606 OID 31473)
-- Dependencies: 1521 1521 1825
-- Name: user_creator_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_creator_fk_fkey FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE SET NULL;


--
-- TOC entry 1863 (class 0 OID 0)
-- Dependencies: 3
-- Name: public; Type: ACL; Schema: -; Owner: -
--

REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM postgres;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO PUBLIC;


-- Completed on 2011-03-09 18:38:13 CET

--
-- PostgreSQL database dump complete
--

