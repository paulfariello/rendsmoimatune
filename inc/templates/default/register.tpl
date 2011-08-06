{include file='inc/header-html.tpl' title='New account'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='register.html'}">{getText id='Register'}</a></h2>
{include file='inc/main.tpl'}
    <div class="column-left">
        <h3>{getText id='Register easily'}</h3>
        {include file='inc/oauth-authentication.tpl'}
    </div>
    <div class="column-right">
        {include file='inc/register-form.tpl'}
    </div>
{include file='inc/footer.tpl'}
