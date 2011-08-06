{include file='inc/header-html.tpl' title='Sign in'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/facebook-authentication.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='sign-in.html'}">{getText id='Sign in'}</a></h2>
{include file='inc/main.tpl'}
<div class="column-left">
    <h3>{getText id='Sign in easily'}</h3>
    {include file='inc/oauth-authentication.tpl'}
</div>
<div class="column-right">
    <h3>{getText id='Sign in in old school way'}</h3>
    {include file='inc/basic-authentication-form.tpl'}
</div>
<div class="clear">
    {include file='inc/register-form.tpl'}
</div>
{include file='inc/footer.tpl'}
