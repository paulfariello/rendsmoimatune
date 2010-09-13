{if isset($message)}
{if $message.type == 'exception'}
<p class="msg error">{$message.exception->getMessage()} on {$message.exception->getFile()} line {$message.exception->getLine()}</p>
{else}
<p class="msg {$message.type}">{$message.content}</p>
{/if}
{/if}