{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a> &raquo; <a href="{$currentAccount->getUrlExpendituresList()}">{getText id="Expenditures"}</a></h2>
{include file='inc/main.tpl'}

                	
<h3>{getText id='Expenditures'}</h3>
    {include file="inc/expenditure-list.tpl" expenditures=$currentAccount->getExpenditures()}
    <a href="{$currentAccount->getUrlNewExpenditure()}" class="button add">{getText id="Add"}</a>
{include file='inc/footer.tpl'}
