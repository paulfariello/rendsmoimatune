{include file="inc/response-header.tpl" status="error"}
    <code>{$errorCode}</code>
    {if isset($errorDesc)}
        <desc>{$errorDesc}</desc>
    {/if}
{include file="inc/response-footer.tpl"}
