{include file='inc/header-html.tpl' title='New account'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-sign-in.tpl'}
<h2><a href="{makeUrl url='new-account.html'}">{getText id='New account'}</a></h2>
{include file='inc/main.tpl'}
          {include file='inc/message.tpl'}
					<h3>{getText id='New account'}</h3>
                	<form action="{makeUrl url='new-account.html'}" class="jNice" method="post" >
                    	<fieldset>
                        	<p><label>{getText id='Email'}</label><input type="text" class="text-long" name="email" /></p>
                        	<p><label>{getText id='Password'}</label><input type="password" class="text-long" name="password" /></p>
                        	<p><label>{getText id='Password Confirm'}</label><input type="password" class="text-long" name="password-confirm" /></p>
                        	<p><label>{getText id='Name'}</label><input type="text" class="text-long" name="name" /></p>
                            <input type="submit" name="create-new-account" value="{getText id='Create new account'}" />
                        </fieldset>
                    </form>
{include file='inc/footer.tpl'}
