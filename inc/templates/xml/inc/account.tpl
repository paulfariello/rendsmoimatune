<account id="{$account->getId()}" title="{$account->getName()|xmlProtect}">
	<users>
		{foreach from=$account->getUsers() item="user"}
			<user id="{$user->getId()}" name="{$user->getName()|xmlProtect}"/>
		{/foreach}
	</users>
	<expenditures>
		{foreach from=$account->getExpenditures() item="expenditure"}
			<expenditure id="{$expenditure->getId()}" title="{$expenditure->getTitle()|xmlProtect}" amount="{$expenditure->getAmount()}"/>
		{/foreach}
	</expenditures>
	<repayments>
		{foreach from=$account->getRepayments() item="repayment"}
			<repayment id="{$repayment->getId()}" from="{$repayment->getPayer()->getId()}" to="{$repayment->getBeneficiary()->getId()}" amount="{$repayment->getAmount()}"/>
		{/foreach}
	</repayments>
</account>
