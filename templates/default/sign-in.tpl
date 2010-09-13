{include file='inc/header-html.tpl' title='Sign in'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
{include file='inc/breadcrumbs.tpl'}
{include file='inc/main.tpl'}

					<h3>{getText id='Sign in'}</h3>
                	<form action="{makeUrl url='sign-in.html'}" class="jNice">
                    	<fieldset>
                        	<p><label>{getText id='Email'}</label><input type="text" class="text-long" /></p>
                        	<p><label>{getText id='Password'}</label><input type="password" class="text-long" /></p>
                            <input type="submit" value="{getText id='Sign in'}" />
                        </fieldset>
                    </form>
{include file='inc/footer.tpl'}
