{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-account.tpl'}
<h2><a href="{makeUrl url='my-account/'}">{getText id='My account'}</a> &raquo; <a href="{makeUrl url='my-account/manage-users.html'}">{getText id='Manage users'}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='Manage users'}</h3>
<form method="post" action="{makeUrl url="my-account/manage-users.html"}">
    <table>
        <thead>
            <tr>
                <td>{getText id="Id"}</td>
                <td>{getText id="First name"}</td>
                <td>{getText id="Last name"}</td>
                <td>{getText id="Delete"}</td>
                <td>{getText id="Invite"}</td>
            </tr>
        </thead>
        <tbody>
            {foreach from=$currentUser->getCreatedUsers() item="user"}
                <tr>
                    <td>{$user->getId()}</td>
                    <td><input type="text" name="update[{$user->getId()}][first-name]" value="{$user->getFirstName()|htmlProtect}" /></td>
                    <td><input type="text" name="update[{$user->getId()}][last-name]" value="{$user->getLastName()|htmlProtect}" /></td>
                    <td><input type="checkbox" name="delete[{$user->getId()}]" value="delete" /></td>
                    <td><a href="{$user->getUrlInvite()}">{getText id="Invite"}</a></td>
                </tr>
            {/foreach}
        </tbody>
    </table>
    <input type="submit" name="update-users" value="{getText id="Update"}" />
    <input type="reset" value="{getText id="Reset"}" />
    <input type="submit" name="delete-users" value="{getText id="Delete selected users"}" />
</form>
{include file='inc/footer.tpl'}
