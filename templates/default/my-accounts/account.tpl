{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a></h2>
{include file='inc/main.tpl'}

                	
<h3>{getText id='Recent expenditures'}</h3>
    {include file="inc/expenditure-list.tpl" expenditures=$currentAccount->getExpenditures(5)}
    <a href="{$currentAccount->getUrlNewExpenditure()}" class="button add">{getText id="New expenditure"}</a>
<h3>{getText id='Recent repayments'}</h3>
    {include file="inc/repayment-list.tpl" repayments=$currentAccount->getRepayments(5)}
    <a href="{$currentAccount->getUrlNewRepayment()}" class="button add">{getText id="New repayment"}</a>
{if $currentAccount->isCreator($currentUser)}
<h3>{getText id='Actions'}</h3>
    <a href="{$currentAccount->getUrlDelete()}" class="button delete">{getText id="Delete this account"}</a>
{/if}
{include file='inc/footer.tpl'}
