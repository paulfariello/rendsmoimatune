BEGIN;

CREATE FUNCTION floattoint(float) RETURNS integer AS $$
        SELECT cast($1 * 100 as integer);
$$ LANGUAGE SQL;

ALTER TABLE expenditure ALTER COLUMN amount TYPE integer USING floattoint(amount);
ALTER TABLE payer       ALTER COLUMN amount TYPE integer USING floattoint(amount);
ALTER TABLE repayment   ALTER COLUMN amount TYPE integer USING floattoint(amount);
ALTER TABLE beneficiary ALTER COLUMN amount TYPE float;
UPDATE beneficiary SET amount = amount * 100;

DROP FUNCTION floattoint(float);

COMMIT;
