{include file='inc/header-html.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2>
    <a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a>
     &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a>
     &raquo; <a href="{$expenditure->getUrlView()}">{$expenditure->getTitle()|htmlProtect}</a>
 </h2>
{include file='inc/main.tpl'}
<h3>{getText id='View expenditure'}</h3>
<dl>
    <dt>{getText id='Title'} :</dt>
    <dd>{$expenditure->getTitle()|htmlProtect}</dd>
    <dt>{getText id='Date'} :</dt>
    <dd>{$expenditure->getDate()->format('l j F Y')}</dd>
    <dt>{getText id='Expenditure amount'} :</dt>
    <dd>{$expenditure->getAmount()|moneyFormat}</dd>
    <dt>{getText id='Payers'} :</dt>
    <dd>
        <ul>
            {foreach from=$expenditure->getPayers() item="payer"}
                <li>
                    <span class="payer-name">{$payer->getUser()->getName()|htmlProtect}</span> (<span class="payer-amount">{$payer->getAmount()|moneyFormat}</span>)
                </li>
            {/foreach}
        </ul>
    </dd>
    <dt>{getText id='Beneficiaries'} :</dt>
    <dd>
        <ul>
            {foreach from=$expenditure->getBeneficiaries() item="beneficiary"}
                <li>
                    <span class="beneficiary-name">{$beneficiary->getUser()->getName()|htmlProtect}</span>
                </li>
            {/foreach}
        </ul>
    </dd>
</dl>
{include file='inc/footer.tpl'}
