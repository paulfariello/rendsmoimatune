{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()}</a> &raquo; <a href="{$currentEvent->getUrlCashUp()}">{getText id='Cash-up'}</a></h2>
{include file='inc/main.tpl'}

{include file='inc/footer.tpl'}