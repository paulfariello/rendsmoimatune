{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-parameters.tpl'}
<h2><a href="{makeUrl url='my-parameters/'}">{getText id='My parameters'}</a> &raquo; <a href="{makeUrl url='my-parameters/send-invitation.html'}">{getText id='Send invitation'}</a></h2>
{include file='inc/main.tpl'}
{if !empty($users)}
    <div class="column-left">
    <h3>{getText id='Users you have created'}</h3>
    <form action="{makeUrl url='my-parameters/send-invitation.html'}" class="jNice" method="post">
        <fieldset>
            {foreach from=$users item='user'}
                <p><label for="email[{$user->getId()}]">{$user->getName()}</label><input type="text" class="text-long" name="email[{$user->getId()}]" value="" /></p>
            {/foreach}
            <input type="submit" name="send-invitation" value="{getText id='Send'}" />
        </fieldset>
    </form>
    </div>
    <div class="column-right">
{/if}
<h3>{getText id='New user'}</h3>
<form action="{makeUrl url='my-parameters/send-invitation.html'}" class="jNice" method="post">
    <fieldset>
        <p><label for="name">{getText id='Name'}</label><input type="text" class="text-long" name="name" value="" /></p>
        <p><label for="email">{getText id='E-mail'}</label><input type="text" class="text-long" name="email" value="" /></p>
        <input type="submit" name="send-invitation-to-new-user" value="{getText id='Send'}" />
    </fieldset>
</form>
{if !empty($users)}
    </div>
{/if}

<h3 class="clear">{getText id='Users you have invited'}</h3>
<form action="{makeUrl url='my-parameters/send-invitation.html'}" class="jNice" method="post">
    <fieldset>
        {foreach from=$invitedUsers item='user'}
            <p><label for="email[{$user->getId()}]">{$user->getName()}</label><input type='checkbox' name="invite[]" value="{$user->getId()}" class="checkbox"/><input type="text" class="text-long" name="email[{$user->getId()}]" value="{$user->getEmail()}" /><span class="invited-user"></span></p>
        {/foreach}
        <input type="submit" name="resend-invitation" value="{getText id='Send again'}" />
    </fieldset>
</form>
{include file='inc/footer.tpl'}
