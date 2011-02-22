{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()|htmlProtect}</a> &raquo; <a href="{$currentEvent->getUrlCashUp()}">{getText id='Cash-up'}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='Balance of payments'}</h3>
<h4 class="balance-due">
    {getText id="Not paid enough"}
</h4>
<h4 class="balance-payed">
    {getText id="Overpaid"}
</h4>
<div class="clear"></div>
{assign var="totalExpenditure" value=$currentEvent->getTotalExpenditure()}
{foreach from=$users item="user"}
    {assign var="balance" value=$currentEvent->getBalance($user)}
    <div class="balance-due">
        {if $balance < 0}
        <div class="balance-bar" style="width: {round(- $balance / $totalExpenditure * 100)}%;">
            {$balance} €
        </div>

        {/if}
    </div>
    <div class="balance-payed">
        {if $balance > 0}
            <div class="balance-bar" style="width: {round($balance / $totalExpenditure * 100)}%;">
                {$balance} €
            </div>
        {/if}
    </div>
    <div class="balance-name">{$user->getName()|htmlProtect}</div>
{/foreach}

<h3>{getText id='Balancing'}</h3>
<ul>
    {foreach from=$debts item="debt"}
        <li>{getText id="%1\$s gives %2\$.2f€ to %3\$s" arg1=$debt->getFrom()->getName() arg2=$debt->getAmount() arg3=$debt->getTo()->getName()}</li>
    {/foreach}
</ul>
{include file='inc/footer.tpl'}
