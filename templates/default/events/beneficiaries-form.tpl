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
