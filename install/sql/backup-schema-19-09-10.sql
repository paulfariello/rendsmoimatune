--
-- PostgreSQL database dump
--

-- Started on 2010-09-19 11:51:04 CEST

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
-- TOC entry 1519 (class 1259 OID 23065)
-- Dependencies: 3
-- Name: event; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE event (
    event_pk integer NOT NULL,
    name character varying,
    start_date date,
    end_date date
);


ALTER TABLE public.event OWNER TO rendsmoimatune;

--
-- TOC entry 1518 (class 1259 OID 23063)
-- Dependencies: 3 1519
-- Name: event_event_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE event_event_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.event_event_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1857 (class 0 OID 0)
-- Dependencies: 1518
-- Name: event_event_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE event_event_pk_seq OWNED BY event.event_pk;


--
-- TOC entry 1524 (class 1259 OID 23108)
-- Dependencies: 3
-- Name: event_user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE event_user (
    user_pcfk integer NOT NULL,
    event_pcfk integer NOT NULL
);


ALTER TABLE public.event_user OWNER TO rendsmoimatune;

--
-- TOC entry 1528 (class 1259 OID 23136)
-- Dependencies: 3
-- Name: expenditure; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure (
    expenditure_pk integer NOT NULL,
    event_fk integer NOT NULL,
    name character varying,
    date date,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.expenditure OWNER TO rendsmoimatune;

--
-- TOC entry 1527 (class 1259 OID 23134)
-- Dependencies: 3 1528
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE expenditure_expenditure_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.expenditure_expenditure_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1858 (class 0 OID 0)
-- Dependencies: 1527
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE expenditure_expenditure_pk_seq OWNED BY expenditure.expenditure_pk;


--
-- TOC entry 1529 (class 1259 OID 23150)
-- Dependencies: 3
-- Name: expenditure_tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure_tag (
    expenditure_pcfk integer NOT NULL,
    tag_pcfk integer NOT NULL
);


ALTER TABLE public.expenditure_tag OWNER TO rendsmoimatune;

--
-- TOC entry 1531 (class 1259 OID 23168)
-- Dependencies: 3
-- Name: involved; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE involved (
    involved_pk integer NOT NULL,
    expenditure_fk integer NOT NULL,
    user_fk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.involved OWNER TO rendsmoimatune;

--
-- TOC entry 1530 (class 1259 OID 23166)
-- Dependencies: 3 1531
-- Name: involved_involved_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE involved_involved_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.involved_involved_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1859 (class 0 OID 0)
-- Dependencies: 1530
-- Name: involved_involved_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE involved_involved_pk_seq OWNED BY involved.involved_pk;


--
-- TOC entry 1533 (class 1259 OID 23188)
-- Dependencies: 3
-- Name: payed; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE payed (
    payed_pk integer NOT NULL,
    expenditure_fk integer NOT NULL,
    user_fk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.payed OWNER TO rendsmoimatune;

--
-- TOC entry 1532 (class 1259 OID 23186)
-- Dependencies: 3 1533
-- Name: payed_payed_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE payed_payed_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.payed_payed_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1860 (class 0 OID 0)
-- Dependencies: 1532
-- Name: payed_payed_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE payed_payed_pk_seq OWNED BY payed.payed_pk;


--
-- TOC entry 1523 (class 1259 OID 23087)
-- Dependencies: 3
-- Name: repayment; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE repayment (
    repayment_pk integer NOT NULL,
    event_fk integer NOT NULL,
    from_user_fk integer NOT NULL,
    to_user_fk integer NOT NULL,
    date date,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.repayment OWNER TO rendsmoimatune;

--
-- TOC entry 1522 (class 1259 OID 23085)
-- Dependencies: 3 1523
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE repayment_repayment_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.repayment_repayment_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1861 (class 0 OID 0)
-- Dependencies: 1522
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE repayment_repayment_pk_seq OWNED BY repayment.repayment_pk;


--
-- TOC entry 1526 (class 1259 OID 23125)
-- Dependencies: 3
-- Name: tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE tag (
    tag_pk integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.tag OWNER TO rendsmoimatune;

--
-- TOC entry 1525 (class 1259 OID 23123)
-- Dependencies: 3 1526
-- Name: tag_tag_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE tag_tag_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.tag_tag_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1862 (class 0 OID 0)
-- Dependencies: 1525
-- Name: tag_tag_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE tag_tag_pk_seq OWNED BY tag.tag_pk;


--
-- TOC entry 1521 (class 1259 OID 23076)
-- Dependencies: 3
-- Name: user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE "user" (
    user_pk integer NOT NULL,
    email character varying NOT NULL,
    password character varying,
    first_name character varying,
    last_name character varying,
    is_admin boolean
);


ALTER TABLE public."user" OWNER TO rendsmoimatune;

--
-- TOC entry 1520 (class 1259 OID 23074)
-- Dependencies: 1521 3
-- Name: user_user_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE user_user_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.user_user_pk_seq OWNER TO rendsmoimatune;

--
-- TOC entry 1863 (class 0 OID 0)
-- Dependencies: 1520
-- Name: user_user_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE user_user_pk_seq OWNED BY "user".user_pk;


--
-- TOC entry 1811 (class 2604 OID 23068)
-- Dependencies: 1518 1519 1519
-- Name: event_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE event ALTER COLUMN event_pk SET DEFAULT nextval('event_event_pk_seq'::regclass);


--
-- TOC entry 1815 (class 2604 OID 23139)
-- Dependencies: 1528 1527 1528
-- Name: expenditure_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE expenditure ALTER COLUMN expenditure_pk SET DEFAULT nextval('expenditure_expenditure_pk_seq'::regclass);


--
-- TOC entry 1816 (class 2604 OID 23171)
-- Dependencies: 1530 1531 1531
-- Name: involved_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE involved ALTER COLUMN involved_pk SET DEFAULT nextval('involved_involved_pk_seq'::regclass);


--
-- TOC entry 1817 (class 2604 OID 23191)
-- Dependencies: 1533 1532 1533
-- Name: payed_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE payed ALTER COLUMN payed_pk SET DEFAULT nextval('payed_payed_pk_seq'::regclass);


--
-- TOC entry 1813 (class 2604 OID 23090)
-- Dependencies: 1523 1522 1523
-- Name: repayment_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE repayment ALTER COLUMN repayment_pk SET DEFAULT nextval('repayment_repayment_pk_seq'::regclass);


--
-- TOC entry 1814 (class 2604 OID 23128)
-- Dependencies: 1525 1526 1526
-- Name: tag_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE tag ALTER COLUMN tag_pk SET DEFAULT nextval('tag_tag_pk_seq'::regclass);


--
-- TOC entry 1812 (class 2604 OID 23079)
-- Dependencies: 1521 1520 1521
-- Name: user_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE "user" ALTER COLUMN user_pk SET DEFAULT nextval('user_user_pk_seq'::regclass);


--
-- TOC entry 1819 (class 2606 OID 23073)
-- Dependencies: 1519 1519
-- Name: event_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY event
    ADD CONSTRAINT event_pkey PRIMARY KEY (event_pk);


--
-- TOC entry 1825 (class 2606 OID 23223)
-- Dependencies: 1524 1524 1524
-- Name: event_user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_pkey PRIMARY KEY (user_pcfk, event_pcfk);


--
-- TOC entry 1829 (class 2606 OID 23144)
-- Dependencies: 1528 1528
-- Name: expenditure_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_pkey PRIMARY KEY (expenditure_pk);


--
-- TOC entry 1831 (class 2606 OID 23154)
-- Dependencies: 1529 1529 1529
-- Name: expenditure_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_pkey PRIMARY KEY (expenditure_pcfk, tag_pcfk);


--
-- TOC entry 1833 (class 2606 OID 23175)
-- Dependencies: 1531 1531 1531
-- Name: involved_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY involved
    ADD CONSTRAINT involved_expenditure_fk_key UNIQUE (expenditure_fk, user_fk);


--
-- TOC entry 1835 (class 2606 OID 23173)
-- Dependencies: 1531 1531
-- Name: involved_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY involved
    ADD CONSTRAINT involved_pkey PRIMARY KEY (involved_pk);


--
-- TOC entry 1838 (class 2606 OID 23195)
-- Dependencies: 1533 1533 1533
-- Name: payed_expenditure_fk_key; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY payed
    ADD CONSTRAINT payed_expenditure_fk_key UNIQUE (expenditure_fk, user_fk);


--
-- TOC entry 1840 (class 2606 OID 23193)
-- Dependencies: 1533 1533
-- Name: payed_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY payed
    ADD CONSTRAINT payed_pkey PRIMARY KEY (payed_pk);


--
-- TOC entry 1823 (class 2606 OID 23092)
-- Dependencies: 1523 1523
-- Name: repayment_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_pkey PRIMARY KEY (repayment_pk);


--
-- TOC entry 1827 (class 2606 OID 23133)
-- Dependencies: 1526 1526
-- Name: tag_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY tag
    ADD CONSTRAINT tag_pkey PRIMARY KEY (tag_pk);


--
-- TOC entry 1821 (class 2606 OID 23084)
-- Dependencies: 1521 1521
-- Name: user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (user_pk);


--
-- TOC entry 1836 (class 1259 OID 23216)
-- Dependencies: 1533
-- Name: fki_; Type: INDEX; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE INDEX fki_ ON payed USING btree (user_fk);


--
-- TOC entry 1844 (class 2606 OID 23224)
-- Dependencies: 1818 1524 1519
-- Name: event_user_event_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_event_pcfk_fkey FOREIGN KEY (event_pcfk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1845 (class 2606 OID 23229)
-- Dependencies: 1820 1521 1524
-- Name: event_user_user_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_user_pcfk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1846 (class 2606 OID 23234)
-- Dependencies: 1519 1528 1818
-- Name: expenditure_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1847 (class 2606 OID 23239)
-- Dependencies: 1828 1529 1528
-- Name: expenditure_tag_expenditure_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_expenditure_pcfk_fkey FOREIGN KEY (expenditure_pcfk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1848 (class 2606 OID 23244)
-- Dependencies: 1526 1529 1826
-- Name: expenditure_tag_tag_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure_tag
    ADD CONSTRAINT expenditure_tag_tag_pcfk_fkey FOREIGN KEY (tag_pcfk) REFERENCES tag(tag_pk) ON DELETE CASCADE;


--
-- TOC entry 1849 (class 2606 OID 23176)
-- Dependencies: 1528 1828 1531
-- Name: involved_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY involved
    ADD CONSTRAINT involved_expenditure_fk_fkey FOREIGN KEY (expenditure_fk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1850 (class 2606 OID 23181)
-- Dependencies: 1521 1820 1531
-- Name: involved_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY involved
    ADD CONSTRAINT involved_user_fk_fkey FOREIGN KEY (user_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1851 (class 2606 OID 23206)
-- Dependencies: 1828 1528 1533
-- Name: payed_expenditure_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY payed
    ADD CONSTRAINT payed_expenditure_fk_fkey FOREIGN KEY (expenditure_fk) REFERENCES expenditure(expenditure_pk) ON DELETE CASCADE;


--
-- TOC entry 1852 (class 2606 OID 23211)
-- Dependencies: 1820 1521 1533
-- Name: payed_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY payed
    ADD CONSTRAINT payed_user_fk_fkey FOREIGN KEY (user_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1841 (class 2606 OID 23249)
-- Dependencies: 1818 1523 1519
-- Name: repayment_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- TOC entry 1842 (class 2606 OID 23254)
-- Dependencies: 1521 1820 1523
-- Name: repayment_from_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_from_user_fk_fkey FOREIGN KEY (from_user_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1843 (class 2606 OID 23259)
-- Dependencies: 1820 1521 1523
-- Name: repayment_to_user_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY repayment
    ADD CONSTRAINT repayment_to_user_fk_fkey FOREIGN KEY (to_user_fk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- TOC entry 1856 (class 0 OID 0)
-- Dependencies: 3
-- Name: public; Type: ACL; Schema: -; Owner: postgres
--

REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM postgres;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO PUBLIC;


-- Completed on 2010-09-19 11:51:04 CEST

--
-- PostgreSQL database dump complete
--

