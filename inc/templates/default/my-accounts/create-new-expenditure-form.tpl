<h3>{getText id='New expenditure'}</h3>
<form action="{$currentAccount->getUrlNewExpenditure()}" method="post" class="jNice">
    <fieldset>
        <p><label>{getText id='Title'} :</label><input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'title'} user-input-exception{/if}" name="title" value="{if isset($_POST.title)}{$_POST.title|htmlProtect}{/if}" /></p>
        <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date{if isset($userInputException) && $userInputException->getInputName() == 'date'} user-input-exception{/if}" name="date" value="{if isset($_POST.date)}{$_POST.date|htmlProtect}{else}{date('d-m-Y')}{/if}" /></p>
        <p><label class="tips-handler" rel="{getText id="What the expenditure really cost."}">{getText id='Expenditure amount'} :</label><input type="text" id="expenditure-amount" class="text-medium{if isset($userInputException) && ($userInputException->getInputName() == 'amount' || $userInputException->getInputName() == 'amount-payed')} user-input-exception{/if}" name="amount" value="{if isset($_POST.amount)}{$_POST.amount|htmlProtect}{/if}" /><span class="inter-input">€</span></p>
        <!-- Payers -->
        <p><label class="tips-handler" rel="{getText id="Here are everyone who participated in the expenditure."}">{getText id='Payers'} :</label></p>
        <div class="subfieldset">
            <div id="clonable-payer">
                <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount payed'} :</label></p>
                <p>
                    <input type="hidden" name="payersId[]" />
                    <input type="text" class="text-medium payer-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="payersName[]" value="" />
                    <input type="text" class="text-medium payer-amount" name="payersAmount[]" value="" />
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
                            <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount payed'} :</label></p>
                            <p>
                                <input type="hidden" name="payersId[]" value="{$_POST.payersId.{$index}|htmlProtect}" />
                                <input type="text" class="text-medium payer-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="payersName[]" value="{$payerName|htmlProtect}" />
                                <input type="text" class="text-medium payer-amount{if isset($userInputException) && $userInputException->getInputName() == 'amount-payed'} user-input-exception{/if}" name="payersAmount[]" value="{$_POST.payersAmount.{$index}|htmlProtect}" />
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
                    <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label><label class="medium-inline">{getText id='Amount payed'} :</label></p>
                    <p>
                        <input type="hidden" name="payersId[]" />
                        <input type="text" class="text-medium payer-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="payersName[]" value="" />
                        <input type="text" class="text-medium payer-amount" name="payersAmount[]" value="" />
                        <select name="payersMetric[]" class="select">
                            <option value="€">€</option>
                            <option value="%">%</option>
                        </select>
                        <a href="#" class="remove-payer"></a>
                    </p>
                </div>
            {/if}
            <p><a href="#" id="add-new-payer" class="button">{getText id="Another payer"}</a></p>
        </div>
        <!-- Beneficiaries -->
        <p><label class="tips-handler" rel="{getText id="Here are everyone who enjoy the expenditure. In other words those who should pay the cost one way or another. Don't forget to also add those who paid."}">{getText id='Beneficiaries'} :</label></p>
        <div class="subfieldset">
            <div id="clonable-beneficiary">
                <p class="inline-label"><label class="medium-inline">{getText id='Name'} :</label></p>
                <p>
                    <input type="hidden" name="beneficiariesId[]" />
                    <input type="text" class="text-medium beneficiary-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="beneficiariesName[]" value="" />
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
                                <input type="text" class="text-medium beneficiary-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="beneficiariesName[]" value="{$beneficiaryName|htmlProtect}" />
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
                        <input type="text" class="text-medium beneficiary-name" rel="{$currentAccount->getUrlAutocompleteUser()}" name="beneficiariesName[]" value="" />
                        <a href="#" class="remove-beneficiary"></a>
                    </p>
                </div>
            {/if}
            <p><a href="#" id="add-new-beneficiary" class="button">{getText id="Another beneficiary"}</a></p>
            <p><a href="#" id="add-all-participants" class="button" rel="{$currentAccount->getUrlLoadAllParticipants()}">{getText id="Add all participants"}</a></p>
        </div>

        <input type="submit" name="create-new-expenditure" value="{getText id='Create'}" />
    </fieldset>
</form>
