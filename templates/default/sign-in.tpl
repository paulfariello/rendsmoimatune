{include file='inc/header-html.tpl' title='Sign in'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='sign-in.html'}">{getText id='Sing-in'}</a></h2>
{include file='inc/main.tpl'}
{include file='inc/message.tpl'}
<h3>{getText id='Sign in'}</h3>
<form action="{makeUrl url='sign-in.html'}" class="jNice" method="post">
    <fieldset>
        <p><label>{getText id='Email'}</label><input type="text" class="text-long" name="email" /></p>
        <p><label>{getText id='Password'}</label><input type="password" class="text-long" name="password" /></p>
        <input type="submit" name="sign-in" value="{getText id='Sign in'}" />
    </fieldset>
</form>
{include file='inc/footer.tpl'}
