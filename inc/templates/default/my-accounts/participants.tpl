{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/merge-select.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a> &raquo; <a href="{$currentAccount->getUrlParticipants()}">{getText id="Participants"}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='Participants list'}</h3>
<form method="post" action="{$currentAccount->getUrlParticipants()}">
    <table>
        <thead>
            <tr>
                <td>{getText id="Id"}</td>
                <td>{getText id="Name"}</td>
                <td>{getText id="Merge"}</td>
                <td>{getText id="Exclusion"}</td>
            </tr>
        </thead>
        <tbody>
            {foreach from=$currentAccount->getUsers() item="user"}
                <tr>
                    <td>{$user->getId()}</td>
                    <td>{$user->getName()|htmlProtect}</td>
                    <td><input type="checkbox" name="merge[]" value="{$user->getId()}" /></td>
                    <td><a href="{$currentAccount->getUrlExclusion($user)}">{getText id="Exclude"}</a></td>
                </tr>
            {foreachelse}
                <tr>
                    <td>{getText id="No participants"}</td>
                </tr>
            {/foreach}
        </tbody>
    </table>
    <input type="submit" name="merge-users" value="{getText id="Merge selected users"}" />
</form>
{include file='inc/footer.tpl'}
