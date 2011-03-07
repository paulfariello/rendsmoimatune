<div id="sidebar">
    <ul class="sideNav">
        {if isset($currentUser)}
            {foreach from=$currentUser->getEvents() item='account'}
                {if isset($currentAccount) and $currentAccount->getId() == $account->getId()}
                    <li class="active">
                        <a href="{$account->getUrlDetail()}">{$account->getName()}</a>
                        <ul class="subNav">
                            <li><a href="{$currentAccount->getUrlExpendituresList()}" class="expenditure">{getText id="Expenditures"}</a></li>
                            <li><a href="{$currentAccount->getUrlRepaymentsList()}" class="repayment">{getText id="Repayments"}</a></li>
                            <li><a href="{$currentAccount->getUrlCashUp()}" class="cash-up">{getText id="Cash up"}</a></li>
                        </ul>
                    </li>
                {else}
                    <li><a href="{$account->getUrlDetail()}">{$account->getName()|htmlProtect}</a></li>
                {/if}
            {/foreach}
        {/if}
        <li {if 'my-accounts/create-new-account.html'|isCurrentPage}class="active"{/if}><a href="{makeUrl url='my-accounts/create-new-account.html'}" class="new">{getText id='New account'}</a></li>
    </ul>
<!-- // .sideNav -->
</div>    
<!-- // #sidebar -->
