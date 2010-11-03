<h3>{getText id='New expenditure'}</h3>
<form action="{$currentEvent->getUrlNewExpenditure()}" method="post" class="jNice">
    <fieldset>
        <p><label>{getText id='Title'} :</label><input type="text" class="text-long" name="title" value="{if isset($_POST.title)}{$_POST.title|htmlentities}{/if}" /></p>
        <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date" name="date" value="{if isset($_POST.date)}{$_POST.date|htmlentities}{else}{date('m-d-Y')}{/if}" /></p>
        <p><label>{getText id='Amount'} :</label><input type="text" id="expenditure-amount" class="text-medium" name="amount" value="{if isset($_POST.amount)}{$_POST.amount|htmlentities}{/if}" /></p>
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
                                <input type="hidden" name="beneficiariesId[]" value="{$_POST.beneficiariesId.{$index}|htmlentities}" />
                                <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="{$beneficiaryName|htmlentities}" />
                                <a href="#" class="remove-beneficiary"></a>
                            </p>
                        </div>
                    {/if}
                {/foreach}
            {else}
                <div>
                    <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label></p>
                    <p>
                        <input type="hidden" name="beneficiariesId[]" />
                        <input type="text" class="text-medium beneficiary-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="beneficiariesName[]" value="" />
                        <a href="#" class="remove-beneficiary"></a>
                    </p>
                </div>
            {/if}
            <p><a href="#" id="add-new-beneficiary"></a></p>
        </div>
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
                                <input type="hidden" name="payersId[]" value="{$_POST.payersId.{$index}|htmlentities}" />
                                <input type="text" class="text-medium payer-name" rel="{makeUrl url='ajax/autocomplete-user.php'}" name="payersName[]" value="{$payerName|htmlentities}" />
                                <input type="text" class="text-medium" name="payersAmount[]" value="{$_POST.payersAmount.{$index}|htmlentities}" />
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
                <div>
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
            {/if}
            <p><a href="#" id="add-new-payer"></a></p>
        </div>

        <input type="submit" name="create-new-expenditure" value="{getText id='Create'}" />
    </fieldset>
</form>