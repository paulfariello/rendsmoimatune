{include file='inc/header-html.tpl' title='Sign in'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/facebook-authentication.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='password-recovery.html'}">{getText id='Reset password'}</a></h2>
{include file='inc/main.tpl'}
    <h3>{getText id='Reset my password'}</h3>
    <form action="{makeUrl url='password-recovery.html'}" class="jNice" method="post">
        <fieldset>
            <input type="hidden" name="id" value="{$id}" />
            <input type="hidden" name="token" value="{$token}" />
            <p><label>{getText id='Password'}</label><input type="password" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password"/></p>
            <p><label>{getText id='Password Confirm'}</label><input type="password" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password-confirm"/></p>
            <input type="submit" name="reset-password" value="{getText id='Reset my password'}" />
        </fieldset>
    </form>
{include file='inc/footer.tpl'}
