{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()|htmlProtect}</a> &raquo; <a href="{$currentEvent->getUrlRepaymentsList()}">{getText id="Repayments"}</a></h2>
{include file='inc/main.tpl'}

<h3>{getText id='Repayments'}</h3>
    {include file="inc/repayment-list.tpl" repayments=$currentEvent->getRepayments()}
    <a href="{$currentEvent->getUrlNewRepayment()}" class="button add">{getText id="New repayment"}</a>
{include file='inc/footer.tpl'}
