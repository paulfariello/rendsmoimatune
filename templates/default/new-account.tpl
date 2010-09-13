{include file='inc/header-html.tpl' title='New account'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
{include file='inc/breadcrumbs.tpl'}
{include file='inc/main.tpl'}

					<h3>{getText id='New account'}</h3>
                	<form action="{makeUrl url='new-account.html'}" class="jNice">
                    	<fieldset>
                        	<p><label>{getText id='Email'}</label><input type="text" class="text-long" /></p>
                        	<p><label>{getText id='Password'}</label><input type="password" class="text-long" /></p>
                            <input type="submit" value="{getText id='Create new account'}" />
                        </fieldset>
                    </form>
{include file='inc/footer.tpl'}
