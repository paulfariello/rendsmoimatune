CREATE TABLE accounts (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	name VARCHAR NOT NULL
);

CREATE TABLE users (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
	name VARCHAR NOT NULL
);

CREATE TABLE repayments (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
	date DATE NOT NULL,
	amount INTEGER NOT NULL,
	payer_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
	beneficiary_id UUID NOT NULL REFERENCES users ON DELETE CASCADE
);

CREATE TABLE expenditures (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	account_id UUID NOT NULL REFERENCES accounts ON DELETE CASCADE,
	name VARCHAR NOT NULL,
	date DATE NOT NULL,
	amount INTEGER NOT NULL,
	payer_id UUID NOT NULL REFERENCES users ON DELETE CASCADE
);

CREATE TABLE debts (
	id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
	debtor_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
	expenditure_id UUID NOT NULL REFERENCES expenditures ON DELETE CASCADE,
	share INTEGER NOT NULL
);

# Add temporary original_id columns in new tables
ALTER TABLE accounts ADD COLUMN original_id INTEGER;
ALTER TABLE users ADD COLUMN original_id INTEGER;
ALTER TABLE expenditures ADD COLUMN original_id INTEGER;

# Migrate from old tables
INSERT INTO accounts (original_id, id, name) SELECT _id, uid, name FROM account;
INSERT INTO users (original_id, account_id, name) SELECT u._id, a.account_id, u.name FROM "user" as u JOIN accounts as a ON u.account_id = a.original_id;
INSERT INTO repayments (account_id, date, amount, payer_id, beneficiary_id) SELECT a.account_id, r.date, r.amount, p.id, b.id FROM repayment as r JOIN accounts as a ON r.account_id = a.original_id JOIN users as p ON p.original_id = r.payer_id JOIN users as b ON b.original_id = r.beneficiary_id;
INSERT INTO expenditures (account_id, name, date, amount, payer_id) SELECT a.account_id, e.name, e.date, e.amount, p.id FROM expenditure as e JOIN accounts as a ON e.account_id = a.original_id JOIN users as p ON p.original_id = e.payer_id;
INSERT INTO debts (debtor_id, expenditure_id, share) SELECT u.id, e.id, d.share FROM debt as d JOIN users as u ON d.debtor_id = u.original_id JOIN expenditure as e ON e.original_id = d.expenditure_id;

# Remove original_id columns
ALTER TABLE accounts DROP COLUMN original_id INTEGER;
ALTER TABLE users DROP COLUMN original_id INTEGER;
ALTER TABLE expenditures DROP COLUMN original_id INTEGER;

# Remove original tables
DROP TABLE account CASCADE;
