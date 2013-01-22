{include file="inc/header.tpl" status="success"}
	<accounts>
		{foreach from=$accounts item="account"}
			<account id="{$account->getId()}" title="{$account->getName()}"/>
		{/foreach}
	</accounts>
{include file="inc/footer.tpl"}
