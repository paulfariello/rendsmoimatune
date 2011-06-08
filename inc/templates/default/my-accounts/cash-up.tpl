{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a> &raquo; <a href="{$currentAccount->getUrlCashUp()}">{getText id='Cash-up'}</a></h2>
{include file='inc/main.tpl'}
{assign var="totalExpenditure" value=$currentAccount->getTotalExpenditure()}
{*assign var="maxPayedAmount" value=$currentAccount->getMaxPayedAmount()*}
{*assign var="maxOwesAmount" value=$currentAccount->getMaxOwesAmount()*}
<h3>{getText id='Expenditure summary'}</h3>
<h4 class="summary-owes">
    {getText id="Is concerned by"}
</h4>
<h4 class="summary-payed">
    {getText id="Payed"}
</h4>
{foreach from=$users item="user"}
    {assign var="payedAmount" value=$currentAccount->getPayedAmount($user)}
    {assign var="owesAmount" value=$currentAccount->getOwesAmount($user)}
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
{assign var="totalExpenditure" value=$currentAccount->getTotalExpenditure()}
{foreach from=$users item="user"}
    {assign var="balance" value=$currentAccount->getBalance($user)}
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

<h3>{getText id='Repayment summary'}</h3>
<h4 class="summary-owes">
    {getText id="Received"}
</h4>
<h4 class="summary-payed">
    {getText id="Repayed"}
</h4>
{foreach from=$users item="user"}
    {assign var="payedAmount" value=$currentAccount->getRepaymentPayedAmount($user)}
    {assign var="receivedAmount" value=$currentAccount->getRepaymentReceivedAmount($user)}
    <div class="summary-payed-amount">
        <div class="balance-bar gradient-{round($payedAmount / $totalExpenditure * 100, -1)}">
            {$payedAmount} €
        </div>
    </div>
    <div class="summary-owes-amount">
        <div class="balance-bar gradient-{round($receivedAmount / $totalExpenditure * 100, -1)}">
            {$receivedAmount} €
        </div>
    </div>
    <div class="summary-name">{$user->getName()|htmlProtect}</div>
    <div class="clear"></div>
{/foreach}
<h4>{getText id="Total"} : {$totalExpenditure} €</h4>
<h3>{getText id='Balancing'}</h3>
<h4 class="repayer"></h4>
<h4 class="repayment">{getText id="owes"}</h4>
<h4 class="repaid"></h4>
<div class="clear"></div>
    {foreach from=$debts item="debt"}
        <div class="repayment">
            <div class="repayer">{$debt->getFrom()->getName()|htmlProtect}</div>
            <div class="amount-owes">{$debt->getAmount()} €</div>
            <div class="repaid">{$debt->getTo()->getName()|htmlProtect}</div>
            <div class="clear"></div>
        </div>
    {/foreach}
{include file='inc/footer.tpl'}
