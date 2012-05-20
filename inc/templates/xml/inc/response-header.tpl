<response>
    <method>{$method}</method>
    <status>{$status}</status>
    {if isset($authToken)}
        <token>{$authToken}</token>
    {/if}
