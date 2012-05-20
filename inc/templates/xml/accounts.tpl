{include file="inc/response-header.tpl" status="success"}
    <accounts>
        {foreach from=$accounts item="account"}
            <account>
                <id>{$account->getId()}</id>
                <name>{$account->getName()}</name>
            </account>
        {/foreach}
    </accounts>
{include file="inc/response-footer.tpl"}
