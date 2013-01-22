-- show all expenditures payers
SELECT p.payer_pk, u.name, p.amount, e.title, e.amount  
FROM account a 
INNER JOIN expenditure e ON e.account_fk = a.account_pk 
INNER JOIN payer p ON p.expenditure_pcfk = e.expenditure_pk 
INNER JOIN "user" u ON u.user_pk = p.user_pcfk 
WHERE a.account_pk = 42;

-- show all expenditures beneficiaries
SELECT b.beneficiary_pk, u.name, b.amount, e.title, e.amount
FROM account a 
INNER JOIN expenditure e ON e.account_fk = a.account_pk 
INNER JOIN beneficiary b ON b.expenditure_pcfk = e.expenditure_pk 
INNER JOIN "user" u ON u.user_pk = b.user_pcfk 
WHERE a.account_pk = 42;

-- show all repayment
SELECT r.repayment_pk, p.name, r.amount, b.name FROM account a 
INNER JOIN repayment r ON r.account_fk = a.account_pk 
INNER JOIN "user" p ON p.user_pk = r.payer_fk 
INNER JOIN "user" b ON b.user_pk = r.beneficiary_fk 
WHERE a.account_pk = 42;
