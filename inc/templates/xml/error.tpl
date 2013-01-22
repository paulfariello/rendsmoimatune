{include file="inc/header.tpl"}
    <error code="{$apiException->getCode()}" message="{$apiException->getMessage()}">
        <description>{$apiException->getDescription()|xmlProtect}</description>
        {if $apiException->isInternal()}
            <trace>
                {$apiException->getPrevious()->getTraceAsString()}
            </trace>
        {/if}
    </error>
{include file="inc/footer.tpl"}
