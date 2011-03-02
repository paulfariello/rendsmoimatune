--
-- PostgreSQL database dump
--

SET statement_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = off;
SET check_function_bodies = false;
SET client_min_messages = warning;
SET escape_string_warning = off;

--
-- Name: plpgsql; Type: PROCEDURAL LANGUAGE; Schema: -; Owner: postgres
--

CREATE PROCEDURAL LANGUAGE plpgsql;


ALTER PROCEDURAL LANGUAGE plpgsql OWNER TO postgres;

SET search_path = public, pg_catalog;

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: beneficiary; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE beneficiary (
    beneficiary_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.beneficiary OWNER TO rendsmoimatune;

--
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE beneficiary_beneficiary_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.beneficiary_beneficiary_pk_seq OWNER TO rendsmoimatune;

--
-- Name: beneficiary_beneficiary_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE beneficiary_beneficiary_pk_seq OWNED BY beneficiary.beneficiary_pk;


--
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
-- Name: event_event_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE event_event_pk_seq OWNED BY event.event_pk;


--
-- Name: event_user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE event_user (
    user_pcfk integer NOT NULL,
    event_pcfk integer NOT NULL
);


ALTER TABLE public.event_user OWNER TO rendsmoimatune;

--
-- Name: expenditure; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure (
    expenditure_pk integer NOT NULL,
    event_fk integer NOT NULL,
    title character varying,
    date date,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.expenditure OWNER TO rendsmoimatune;

--
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
-- Name: expenditure_expenditure_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE expenditure_expenditure_pk_seq OWNED BY expenditure.expenditure_pk;


--
-- Name: expenditure_tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE expenditure_tag (
    expenditure_pcfk integer NOT NULL,
    tag_pcfk integer NOT NULL
);


ALTER TABLE public.expenditure_tag OWNER TO rendsmoimatune;

--
-- Name: payer; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE payer (
    payer_pk integer NOT NULL,
    expenditure_pcfk integer NOT NULL,
    user_pcfk integer NOT NULL,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.payer OWNER TO rendsmoimatune;

--
-- Name: payer_payer_pk_seq; Type: SEQUENCE; Schema: public; Owner: rendsmoimatune
--

CREATE SEQUENCE payer_payer_pk_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;


ALTER TABLE public.payer_payer_pk_seq OWNER TO rendsmoimatune;

--
-- Name: payer_payer_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE payer_payer_pk_seq OWNED BY payer.payer_pk;


--
-- Name: repayment; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE repayment (
    repayment_pk integer NOT NULL,
    event_fk integer NOT NULL,
    payer_fk integer NOT NULL,
    beneficiary_fk integer NOT NULL,
    date date,
    amount numeric(10,2) NOT NULL
);


ALTER TABLE public.repayment OWNER TO rendsmoimatune;

--
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
-- Name: repayment_repayment_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE repayment_repayment_pk_seq OWNED BY repayment.repayment_pk;


--
-- Name: tag; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE tag (
    tag_pk integer NOT NULL,
    name character varying NOT NULL
);


ALTER TABLE public.tag OWNER TO rendsmoimatune;

--
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
-- Name: tag_tag_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE tag_tag_pk_seq OWNED BY tag.tag_pk;


--
-- Name: user; Type: TABLE; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE TABLE "user" (
    user_pk integer NOT NULL,
    email character varying NOT NULL,
    password character varying,
    first_name character varying,
    last_name character varying,
    is_admin boolean,
    registered boolean DEFAULT true
);


ALTER TABLE public."user" OWNER TO rendsmoimatune;

--
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
-- Name: user_user_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: rendsmoimatune
--

ALTER SEQUENCE user_user_pk_seq OWNED BY "user".user_pk;


--
-- Name: beneficiary_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE beneficiary ALTER COLUMN beneficiary_pk SET DEFAULT nextval('beneficiary_beneficiary_pk_seq'::regclass);


--
-- Name: event_pk; Type: DEFAULT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE event ALTER COLUMN event_pk SET DEFAULT nextval('event_event_pk_seq'::regclass);


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
-- Name: event_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY event
    ADD CONSTRAINT event_pkey PRIMARY KEY (event_pk);


--
-- Name: event_user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_pkey PRIMARY KEY (user_pcfk, event_pcfk);


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
-- Name: user_pkey; Type: CONSTRAINT; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

ALTER TABLE ONLY "user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (user_pk);


--
-- Name: fki_; Type: INDEX; Schema: public; Owner: rendsmoimatune; Tablespace: 
--

CREATE INDEX fki_ ON payer USING btree (user_pcfk);


--
-- Name: event_user_event_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_event_pcfk_fkey FOREIGN KEY (event_pcfk) REFERENCES event(event_pk) ON DELETE CASCADE;


--
-- Name: event_user_user_pcfk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY event_user
    ADD CONSTRAINT event_user_user_pcfk_fkey FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE CASCADE;


--
-- Name: expenditure_event_fk_fkey; Type: FK CONSTRAINT; Schema: public; Owner: rendsmoimatune
--

ALTER TABLE ONLY expenditure
    ADD CONSTRAINT expenditure_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


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
    ADD CONSTRAINT repayment_event_fk_fkey FOREIGN KEY (event_fk) REFERENCES event(event_pk) ON DELETE CASCADE;


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
-- Name: public; Type: ACL; Schema: -; Owner: postgres
--

REVOKE ALL ON SCHEMA public FROM PUBLIC;
REVOKE ALL ON SCHEMA public FROM postgres;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

