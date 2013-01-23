{include file='inc/header-html.tpl' title='Exclude'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2>
    <a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a>
    &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a>
    &raquo; <a href="{$currentAccount->getUrlParticipants()}">{getText id="Participants"}</a>
    &raquo; {getText id="Exclude"}
</h2>
{include file='inc/main.tpl'}
    <h3>{getText id='Exclude %1$s from %2$s' arg1=$user->getName() arg2=$currentAccount->getName()}</h3>
    <form action="{$currentAccount->getUrlExclusion($user)}" method="post">
        <p class="msg warning">{getText id="P_UserExclusionWarning" arg1=$user->getName() arg2=$currentAccount->getName()}</p>
        <input type="hidden" name="csrf-token" value="{generateCSRFToken id="confirm-exclusion"}" />
        <input type="submit" name="confirm-exclusion" value="{getText id="Yes, exclude it"}" />
    </form>
{include file='inc/footer.tpl'}
