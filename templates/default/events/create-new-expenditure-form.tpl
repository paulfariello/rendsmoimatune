<h3>{getText id='New expenditure'}</h3>
<form action="{$currentEvent->getUrlNewExpenditure()}" method="post" class="jNice">
    <fieldset>
        <p><label>{getText id='Name'} :</label><input type="text" class="text-long" name="name" value="{if isset($_POST.name)}{$_POST.name|htmlentities}{/if}" /></p>
        <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date" name="date" value="{if isset($_POST.date)}{$_POST.date|htmlentities}{else}{date('m-d-Y')}{/if}" /></p>
        <p><label>{getText id='Amount'} :</label><input type="text" class="text-medium" name="amount" value="{if isset($_POST.amount)}{$_POST.amount|htmlentities}{/if}" /></p>
        <p><label>{getText id='Payers'} :</label><input type="text" class="text-long textboxuserlist" name="payers" value="{if isset($_POST.payers)}{$_POST.payers|htmlentities}{/if}" /></p>
        <p><label>{getText id='Involved users'} :</label><input type="text" class="text-long textboxuserlist" name="involved" value="{if isset($_POST.involved)}{$_POST.involved|htmlentities}{/if}" /></p>
        <input type="submit" name="create-new-expenditure" value="Add expenditure" />
    </fieldset>
</form>