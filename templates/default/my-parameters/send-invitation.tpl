{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-parameters.tpl'}
<h2><a href="{makeUrl url='my-parameters/'}">{getText id='My account'}</a> &raquo; <a href="{makeUrl url='my-parameters/send-invitation.html'}">{getText id='Send invitation'}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='Users you have created'}</h3>
<form action="{makeUrl url='my-parameters/send-invitation.html'}" class="jNice" method="post">
    <fieldset>
        {foreach from=$users item='user'}
            <p><label for="email[{$user->getId()}]">{$user->getName()}</label><input type="text" class="text-long" name="email[{$user->getId()}]" value="" /></p>
        {/foreach}
        <input type="submit" name="send-invitation" value="{getText id='Send'}" />
    </fieldset>
</form>

<h3>{getText id='Users you have invited'}</h3>
<form action="{makeUrl url='my-parameters/send-invitation.html'}" class="jNice" method="post">
    <fieldset>
        {foreach from=$invitedUsers item='user'}
            <p><label for="email[{$user->getId()}]">{$user->getName()}</label><input type='checkbox' name="invite[]" value="{$user->getId()}" class="checkbox"/><input type="text" class="text-long" name="email[{$user->getId()}]" value="{$user->getEmail()}" /><span class="invited-user"></span></p>
        {/foreach}
        <input type="submit" name="resend-invitation" value="{getText id='Send again'}" />
    </fieldset>
</form>
{include file='inc/footer.tpl'}
