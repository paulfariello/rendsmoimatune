{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-account.tpl'}
<h2><a href="{makeUrl url='my-account/'}">{getText id='My account'}</a></h2>
{include file='inc/main.tpl'}
<h3>{getText id='My account'}</h3>
Bienvenue {$currentUser->getName()} (id={$currentUser->getId()})
{include file='inc/footer.tpl'}