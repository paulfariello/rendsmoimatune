{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-index.tpl'}
{include file='inc/main.tpl'}
{include file='inc/message.tpl'}
<h3>{getText id='Request API key'}</h3>
{if isset($client)}
<p>
    {if $client->getApiKey() != null}
        {getText id="Congratulations ! You are now able to use our fantastic API with the following key."}
        <code>{$client->getApiKey()}</code>
    {else}
        {getText id="An email has been sent to you. Please read it to get your api key."}
    {/if}
</p>
{else}
<form action="{makeUrl url='api/register-client.php'}" method="post" class="jNice">
    <fieldset>
        <p>
            <label for="email">{getText id="email"}</label>
            <input type="text" value="" name="email" />
        </p>
        <p>
            <input type="submit" value="{getText id="Register"}" name="register-client" />
        </p>
    </fieldset>
</form>
{/if}

{include file='inc/footer.tpl'}
