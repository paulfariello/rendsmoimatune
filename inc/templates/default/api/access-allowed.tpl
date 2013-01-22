{include file='inc/header-html.tpl'}
    <meta http-equiv="refresh" content="5; URL={$callback|htmlProtect}" />
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-index.tpl'}
{include file='inc/main.tpl'}
        <h3>{getText id='Application authorization'}</h3>
        <p>{getText id='You just grant full read and write access to your data to the following third party.'}</p>
        <div class='oauth-consumer'>
            <h4>{$consumer->getName()|htmlProtect}</h4>
            {if $consumer->getUrl() != ''}
                <p><a href="{$consumer->getUrl()|htmlProtect}">{$consumer->getUrl()|htmlProtect}</a></p>
            {/if}
        </div>
        <p>{getText id='You will be redirected to <a href="%1$s">%1$s</a> in 5 seconds.' arg1={$callback|htmlProtect}}</p>
{include file='inc/footer.tpl'}
