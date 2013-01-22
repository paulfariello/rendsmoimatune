<expenditure id="{$expenditure->getId()}" amount="{$expenditure->getAmount()}" title="{$expenditure->getTitle()|xmlProtect}">
    <payers>
        {foreach from=$expenditure->getPayers() item="payer"}
            <payer id="{$payer->getId()}" user="{$payer->getUser()->getId()}" amount="{$payer->getAmount()}"/>
        {/foreach}
    </payers>
    <beneficiaries>
        {foreach from=$expenditure->getBeneficiaries() item="beneficiary"}
            <beneficiary id="{$beneficiary->getId()}" user="{$beneficiary->getUser()->getId()}" amount="{$beneficiary->getAmount()}"/>
        {/foreach}
    </beneficiaries>
</expenditure>
