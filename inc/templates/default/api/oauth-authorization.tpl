{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-index.tpl'}
{include file='inc/main.tpl'}
        <h3>{getText id='Application authorization'}</h3>
        <p>{getText id='Are you sure you want to grant a full read and write access to the following third party ? You will be able to revoke this access at anytime throught your parameters.'}</p>
        <div class='oauth-consumer'>
            <h4>{$consumer->getName()|htmlProtect}</h4>
            {if $consumer->getUrl() != ''}
                <p><a href="{$consumer->getUrl()|htmlProtect}">{$consumer->getUrl()|htmlProtect}</a></p>
            {/if}
            <p>{getText id='Note that we do not take any responsability concerning what this third party could do with your data.'}</p>
        </div>
        <form action="{makeUrl url="api/oauth_authorization.php"}" method="post">
            <input type="hidden" name="csrf-token" value="{generateCSRFToken id="oauth-authorization"}" />
            <input type="hidden" name="oauth_token" value="{$token->getToken()|htmlProtect}" />
            <input type="submit" name="allow-access" value="{getText id="Yes, grant access"}" />
            <input type="submit" name="deny-access" value="{getText id="No way !"}" />
        </form>
{include file='inc/footer.tpl'}
