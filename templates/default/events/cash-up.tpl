{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()}</a> &raquo; <a href="{$currentEvent->getUrlCashUp()}">{getText id='Cash-up'}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='Balance of payments'}</h3>
{foreach from=$balances item="balance"}
    <div class="balance-due">
        {if $balance.amount < 0}
        <div class="balance-bar" style="width: {round(- $balance.amount / $totalExpenditure * 100)}%;">
            {$balance.amount} €
        </div>
        <!-- <div class="balance-amount">
            {$balance.amount}
        </div> -->
        {/if}
    </div>
    <div class="balance-payed">
        {if $balance.amount > 0}
            <div class="balance-bar" style="width: {round($balance.amount / $totalExpenditure * 100)}%;">
                {$balance.amount} €
            </div>
            <!-- <div class="balance-amount">
                {$balance.amount}
            </div> -->
        {/if}
    </div>
    <div class="balance-name">{$balance.user->getName()}</div>
{/foreach}

<h3>{getText id='Balancing'}</h3>
<ul>
    {foreach from=$debts item="debt"}
        <li>{$debt->getFrom()->getName()} doit {$debt->getAmount()} à {$debt->getTo()->getName()}</li>
    {/foreach}
</ul>
{include file='inc/footer.tpl'}