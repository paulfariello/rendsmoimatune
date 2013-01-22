{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-index.tpl'}
{include file='inc/main.tpl'}
<h3>{getText id='Request API key'}</h3>
{if isset($consumer)}
<p>
    {if $consumer->getKey() != null}
        {getText id="Congratulations ! You are now able to use our fantastic API with the following key and secret."}
        <h4>{getText id="Consumer key"}</h4>
        <code>{$consumer->getKey()}</code>
        <h4>{getText id="Consumer secret"}</h4>
        <code>{$consumer->getSecret()}</code>
    {else}
        {getText id="An email has been sent to you. Please read it to get your api key."}
    {/if}
</p>
{else}
<form action="{makeUrl url='api/register-consumer.php'}" method="post" class="jNice">
    <fieldset>
        <p>
            <label>{getText id='Name'} :</label>
            <input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'name'} user-input-exception{/if}" name="name" value="{if isset($_POST.name)}{$_POST.name|htmlProtect}{/if}" />
        </p>
        <p>
            <label>{getText id='URL'} :</label>
            <input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'url'} user-input-exception{/if}" name="url" value="{if isset($_POST.url)}{$_POST.url|htmlProtect}{/if}" />
        </p>
        <p>
            <label>{getText id='email'} :</label>
            <input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'email'} user-input-exception{/if}" name="email" value="{if isset($_POST.email)}{$_POST.email|htmlProtect}{/if}" />
        </p>
        <p>
            <input type="submit" value="{getText id="Register"}" name="register-consumer" />
        </p>
    </fieldset>
</form>
{/if}

{include file='inc/footer.tpl'}
