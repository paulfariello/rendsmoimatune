BEGIN;

	ALTER TABLE payer DROP CONSTRAINT "payed_user_fk_fkey";
	ALTER TABLE payer ADD CONSTRAINT "payed_user_fk_fkey" FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

	ALTER TABLE beneficiary DROP CONSTRAINT "involved_user_fk_fkey";
	ALTER TABLE beneficiary ADD CONSTRAINT "involved_user_fk_fkey" FOREIGN KEY (user_pcfk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

	ALTER TABLE repayment DROP CONSTRAINT "repayment_to_user_fk_fkey";
	ALTER TABLE repayment ADD CONSTRAINT "repayment_to_user_fk_fkey" FOREIGN KEY (beneficiary_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

	ALTER TABLE repayment DROP CONSTRAINT "repayment_from_user_fk_fkey";
	ALTER TABLE repayment ADD CONSTRAINT "repayment_from_user_fk_fkey" FOREIGN KEY (payer_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

	ALTER TABLE repayment DROP CONSTRAINT "repayment_creator_fk_fkey";
	ALTER TABLE repayment ADD CONSTRAINT "repayment_creator_fk_fkey" FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

	ALTER TABLE expenditure DROP CONSTRAINT "expenditure_creator_fk_fkey";
	ALTER TABLE expenditure ADD CONSTRAINT "expenditure_creator_fk_fkey" FOREIGN KEY (creator_fk) REFERENCES "user"(user_pk) ON DELETE RESTRICT;

COMMIT;
