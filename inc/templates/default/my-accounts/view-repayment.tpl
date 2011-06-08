{include file='inc/header-html.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2>
    <a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a>
     &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a>
     &raquo; <a href="{$repayment->getUrlView()}">{$repayment->getDescription()|htmlProtect}</a>
 </h2>
{include file='inc/main.tpl'}
<h3>{getText id='View repayment'}</h3>
<dl>
    <dt>{getText id='Payer'} :</dt>
    <dd>{$repayment->getPayer()->getName()|htmlProtect}</dd>
    <dt>{getText id='Beneficiary'} :</dt>
    <dd>{$repayment->getBeneficiary()->getName()|htmlProtect}</dd>
    <dt>{getText id='Repayment amount'} :</dt>
    <dd>{$repayment->getAmount()|moneyFormat}</dd>
    <dt>{getText id='Date'} :</dt>
    <dd>{$repayment->getDate()->format('l j F Y')}</dd>
</dl>
{include file='inc/footer.tpl'}
