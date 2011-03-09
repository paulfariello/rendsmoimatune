<h3>{getText id='Edit expenditure'}</h3>
<form action="{$currentAccount->getUrlEditExpenditure($expenditure)}" method="post" class="jNice">
    <fieldset>
        <p><label>{getText id='Title'} :</label><input type="text" class="text-long" name="title" value="{if isset($_POST.title)}{$_POST.title|htmlProtect}{else}{$expenditure->getTitle()|htmlProtect}{/if}" /></p>
        <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date" name="date" value="{if isset($_POST.date)}{$_POST.date|htmlProtect}{else}{$expenditure->getDate()->format('d-m-Y')}{/if}" /></p>
        <p><label>{getText id='Amount'} :</label><input type="text" id="expenditure-amount" class="text-medium" name="amount" value="{if isset($_POST.amount)}{$_POST.amount|htmlProtect}{else}{$expenditure->getAmount()|htmlProtect}{/if}" /></p>
        <!-- Payers -->
        <p><label>{getText id='Payers'} :</label></p>
        <div class="subfieldset">
            <div id="clonable-payer">
                <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount'} :</label></p>
                <p>
                    <input type="hidden" name="payersId[]" />
                    <input type="text" class="text-medium payer-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="payersName[]" value="" />
                    <input type="text" class="text-medium" name="payersAmount[]" value="" />
                    <select name="payersMetric[]" class="select">
                        <option value="€">€</option>
                        <option value="%">%</option>
                    </select>
                    <a href="#" class="remove-payer"></a>
                </p>
            </div>
            {if isset($_POST.payersName)}
                {foreach from=$_POST.payersName key="index" item="payerName"}
                    {if !empty($payerName)}
                        <div>
                            <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount'} :</label></p>
                            <p>
                                <input type="hidden" name="payersId[]" value="{$_POST.payersId.{$index}|htmlProtect}" />
                                <input type="text" class="text-medium payer-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="payersName[]" value="{$payerName|htmlProtect}" />
                                <input type="text" class="text-medium" name="payersAmount[]" value="{$_POST.payersAmount.{$index}|htmlProtect}" />
                                <select name="payersMetric[]" class="select">
                                    <option value="%" {if $_POST.payersMetric.{$index} == '%'}selected="selected"{/if}>%</option>
                                    <option value="€" {if $_POST.payersMetric.{$index} == '€'}selected="selected"{/if}>€</option>
                                </select>
                                <a href="#" class="remove-payer"></a>
                            </p>
                        </div>
                    {/if}
                {/foreach}
            {else}
                {foreach from=$expenditure->getPayers() item="payer"}
                    <div>
                        <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount'} :</label></p>
                        <p>
                            <input type="hidden" name="payersId[]" value="{$payer->getUser()->getId()}" />
                            <input type="text" class="text-medium payer-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="payersName[]" value="{$payer->getUser()->getName()|htmlProtect}" />
                            <input type="text" class="text-medium" name="payersAmount[]" value="{$payer->getAmount()|htmlProtect}" />
                            <select name="payersMetric[]" class="select">
                                <option value="%">%</option>
                                <option value="€" selected="selected">€</option>
                            </select>
                            <a href="#" class="remove-payer"></a>
                        </p>
                    </div>
                {/foreach}
            {/if}
            <p><a href="#" id="add-new-payer"></a></p>
        </div>
        <!-- Beneficiaries -->
        <p><label>{getText id='Beneficiaries'} :</label></p>
        <div class="subfieldset">
            <div id="clonable-beneficiary">
                <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label></p>
                <p>
                    <input type="hidden" name="beneficiariesId[]" />
                    <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="" />
                    <a href="#" class="remove-beneficiary"></a>
                </p>
            </div>
            {if isset($_POST.beneficiariesName)}
                {foreach from=$_POST.beneficiariesName key="index" item="beneficiaryName"}
                    {if !empty($beneficiaryName)}
                        <div>
                            <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label></p>
                            <p>
                                <input type="hidden" name="beneficiariesId[]" value="{$_POST.beneficiariesId.{$index}|htmlProtect}" />
                                <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="{$beneficiaryName|htmlProtect}" />
                                <a href="#" class="remove-beneficiary"></a>
                            </p>
                        </div>
                    {/if}
                {/foreach}
            {else}
                {foreach from=$expenditure->getBeneficiaries() item="beneficiary"}
                    <div>
                        <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label></p>
                        <p>
                            <input type="hidden" name="beneficiariesId[]" value="{$beneficiary->getUser()->getId()}" />
                            <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="{$beneficiary->getUser()->getName()|htmlProtect}" />
                            <a href="#" class="remove-beneficiary"></a>
                        </p>
                    </div>
                {/foreach}
            {/if}
            <p><a href="#" id="add-new-beneficiary"></a></p>
        </div>
        <input type="submit" name="edit-expenditure" value="{getText id='Save'}" />
        <input type="reset" name="edit-expenditure" value="{getText id='Reset'}" />
    </fieldset>
</form>
