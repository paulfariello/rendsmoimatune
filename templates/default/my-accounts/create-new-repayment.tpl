{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-repayment-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='New repayment'}</h3>
<form action="{$currentAccount->getUrlNewRepayment()}" method="post" class="jNice">
    <fieldset>
        <p class="inline-label"><label class="medium-inline">{getText id='Payer\'s name'}</label><label class="medium-inline">{getText id='Beneficiary\'s name'}</label></p>
        <p>
            <input type="hidden" name="payerId" />
            <input type="text" class="text-medium payer-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="payersName[]" value="" />
            <span class="inter-input">gives to</span>
            <input type="hidden" name="beneficiaryId" />
            <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="" />
        </p>
        <p><label>{getText id='Amount'} :</label><input type="text" id="expenditure-amount" class="text-medium" name="amount" value="{if isset($_POST.amount)}{$_POST.amount|htmlProtect}{/if}" /><span class="inter-input">€</span></p>
        <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date" name="date" value="{if isset($_POST.date)}{$_POST.date|htmlProtect}{else}{date('m-d-Y')}{/if}" /></p>
        <input type="submit" name="create-new-repayment" value="{getText id='Create'}" />
    </fieldset>
</form>
{include file='inc/footer.tpl'}
