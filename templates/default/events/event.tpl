{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()}</a></h2>
{include file='inc/main.tpl'}

                	
<h3>{getText id='Expenditures'}</h3>
<table cellpadding="0" cellspacing="0">
    {foreach from=$currentEvent->getExpenditures() item='expenditure'}
        <tr>
            <td>
                {$expenditure->getTitle()}
                {if $expenditure->getPayers()->count() > 0}
                    <span class="payers">
                        {getText id='Payers'} :
                        {foreach from=$expenditure->getPayers() item="payer" name="payers"}
                            {$payer->getUser()->getName()}{if !$smarty.foreach.payers.last},{/if}
                        {/foreach}
                    </span>
                {/if}
                {if $expenditure->getBeneficiaries()->count() > 0}
                    <span class="beneficiaries">
                        {getText id='Beneficiaries'} :
                        {foreach from=$expenditure->getBeneficiaries() item="beneficiary" name="beneficiaries"}
                            {$beneficiary->getUser()->getName()}{if !$smarty.foreach.beneficiaries.last},{/if}
                        {/foreach}
                    </span>
                {/if}
            </td>
            <td class="action"><a href="#" class="view">{getText id='View'}</a><a href="{$expenditure->getUrlEdit()}" class="edit">{getText id='Edit'}</a><a href="{$expenditure->getUrlDelete()}" class="delete">{getText id='Delete'}</a></td>
        </tr>
    {/foreach}
</table>
<a href="{$currentEvent->getUrlNewExpenditure()}" class="button add">{getText id="New expenditure"}</a>
<h3>{getText id='Repayments'}</h3>
<table cellpadding="0" cellspacing="0">
    {foreach from=$currentEvent->getRepayments() item='repayment'}
        <tr>
            <td>{$repayment->getDescription()}</td>
            <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
        </tr>
    {/foreach}
</table>
<a href="{$currentEvent->getUrlNewRepayment()}" class="button add">{getText id="New repayment"}</a>
{include file='inc/footer.tpl'}
