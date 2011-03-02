{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()|htmlProtect}</a> &raquo; <a href="{$currentEvent->getUrlCashUp()}">{getText id='Cash-up'}</a></h2>
{include file='inc/main.tpl'}
{assign var="totalExpenditure" value=$currentEvent->getTotalExpenditure()}
{*assign var="maxPayedAmount" value=$currentEvent->getMaxPayedAmount()*}
{*assign var="maxOwesAmount" value=$currentEvent->getMaxOwesAmount()*}
<h3>{getText id='Summary'}</h3>
<h4 class="summary-owes">
    {getText id="Is concerned by"}
</h4>
<h4 class="summary-payed">
    {getText id="Payed"}
</h4>
{foreach from=$users item="user"}
    {assign var="payedAmount" value=$currentEvent->getPayedAmount($user)}
    {assign var="owesAmount" value=$currentEvent->getOwesAmount($user)}
    <div class="summary-payed-amount">
        <div class="balance-bar gradient-{round($payedAmount / $totalExpenditure * 100, -1)}">
            {$payedAmount} €
        </div>
    </div>
    <div class="summary-owes-amount">
        <div class="balance-bar gradient-{round($owesAmount / $totalExpenditure * 100, -1)}">
            {$owesAmount} €
        </div>
    </div>
    <div class="summary-name">{$user->getName()|htmlProtect}</div>
    <div class="clear"></div>
{/foreach}
<h4>{getText id="Total"} : {$totalExpenditure} €</h4>

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
