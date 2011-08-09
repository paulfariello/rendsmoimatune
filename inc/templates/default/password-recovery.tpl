{include file='inc/header-html.tpl' title='Sign in'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/facebook-authentication.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='password-recovery.html'}">{getText id='Forgot password ?'}</a></h2>
{include file='inc/main.tpl'}
    <h3>{getText id='Recover my password'}</h3>
    <form action="{makeUrl url='password-recovery.html'}" class="jNice" method="post">
        <fieldset>
            <p><label>{getText id='Email'}</label><input type="text" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'email'} user-input-exception{/if}" name="email" value="{if isset($_POST)}{$_POST.email}{/if}"/></p>
            <input type="submit" name="recover-password" value="{getText id='Recover my password'}" />
        </fieldset>
    </form>
{include file='inc/footer.tpl'}
