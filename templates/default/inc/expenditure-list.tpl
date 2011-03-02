<table cellpadding="0" cellspacing="0">
    {foreach from=$expenditures item='expenditure'}
        <tr>
            <td>
                {$expenditure->getTitle()|htmlProtect}
            </td> 
            <td>
                <span class="money">{$expenditure->getAmount()|moneyFormat}</span>
            </td> 
            <td>
                {if $expenditure->getPayers()->count() > 0}
                    <span class="payers">
                        {getText id='Payers'} :
                        {foreach from=$expenditure->getPayers() item="payer" name="payers"}
                            {$payer->getUser()->getName()|htmlProtect}{if !$smarty.foreach.payers.last},{/if}
                        {/foreach}
                    </span>
                {/if}
                {if $expenditure->getBeneficiaries()->count() > 0}
                    <span class="beneficiaries">
                        {getText id='Beneficiaries'} :
                        {foreach from=$expenditure->getBeneficiaries() item="beneficiary" name="beneficiaries"}
                            {$beneficiary->getUser()->getName()|htmlProtect}{if !$smarty.foreach.beneficiaries.last},{/if}
                        {/foreach}
                    </span>
                {/if}
            </td>
            <td class="action"><a href="#" class="view">{getText id='View'}</a><a href="{$expenditure->getUrlEdit()}" class="edit">{getText id='Edit'}</a><a href="{$expenditure->getUrlDelete()}" class="delete">{getText id='Delete'}</a></td>
        </tr>
    {/foreach}
</table>
