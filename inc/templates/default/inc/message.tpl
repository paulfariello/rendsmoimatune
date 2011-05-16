{if isset($messages)}
    {foreach from=$messages item="message"}
        {if $message.type == 'exception'}
            <p class="msg error">{$message.exception->getMessage()} on {$message.exception->getFile()} line {$message.exception->getLine()}</p>
        {else}
            <p class="msg {$message.type}">{$message.content}</p>
        {/if}
    {/foreach}
{/if}