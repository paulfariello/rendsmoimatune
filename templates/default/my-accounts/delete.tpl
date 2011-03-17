{include file='inc/header-html.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2>
    <a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a>
    &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a>
    &raquo; <a href="{$currentAccount->getUrlDelete()}">{getText id='Deletion'}</a>
</h2>
{include file='inc/main.tpl'}
<form action="{$currentAccount->getUrlDelete()}" method="post">
    <p class="msg warning">{getText id="P_AccountDeletionWarning" arg1=$currentAccount->getName()}</p>
    <input type="submit" name="confirm-deletion" value="{getText id="Yes, delete it"}" />
</form>
{include file='inc/footer.tpl'}
