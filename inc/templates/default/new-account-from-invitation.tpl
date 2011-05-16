{include file='inc/header-html.tpl' title='New account'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='new-account.html'}">{getText id='New account'}</a></h2>
{include file='inc/main.tpl'}
    <h3>{getText id='My account'}</h3>
    <form action="{makeUrl url='new-account-invitation.html'}" class="jNice" method="post" >
        <fieldset>
            <input type="hidden" name="id" value="{$user->getId()}" />
            <input type="hidden" name="token" value="{$user->getInvitationToken()}" />
            <p><label>{getText id='Name'}</label><input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'name'} user-input-exception{/if}" name="name" value="{if isset($_POST)}{$_POST.name|htmlProtect}{else}{$user->getName()}{/if}" /></p>
            <p><label>{getText id='Email'}</label><input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'email'} user-input-exception{/if}" name="email" value="{if isset($_POST)}{$_POST.email|htmlProtect}{else}{$user->getEmail()}{/if}" /></p>
            <p><label>{getText id='Password'}</label><input type="password" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password" /></p>
            <p><label>{getText id='Password Confirm'}</label><input type="password" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password-confirm" /></p>
            <input type="submit" name="create-new-account-from-invitation" value="{getText id='Create new account'}" />
        </fieldset>
    </form>
{include file='inc/footer.tpl'}
