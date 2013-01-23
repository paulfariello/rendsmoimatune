{include file='inc/header-html.tpl' title='Exclude'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2>
    <a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a>
    &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a>
    &raquo; <a href="{$currentAccount->getUrlParticipants()}">{getText id="Participants"}</a>
    &raquo; {getText id="Exclude"}
</h2>
{include file='inc/main.tpl'}
    <h3>{getText id='Exclude %1$s from %2$s' arg1=$user->getName() arg2=$currentAccount->getName()}</h3>
    <p class="msg error">
        {getText id='You cannot exclude %1$s from %2$s because he or she is involved in the following expenditures and repayments.' arg1=$user->getName() arg2=$currentAccount->getName()}
    </p>
    {include file='inc/message.tpl'}
    <h3>{getText id='Expenditures'}</h3>
        {include file="inc/expenditure-list.tpl"}
    <h3>{getText id='Repayments'}</h3>
        {include file="inc/repayment-list.tpl"}
{include file='inc/footer.tpl'}
