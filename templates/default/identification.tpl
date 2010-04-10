{include file="inc/headerHtml.tpl"}
<title>Botte De Foin : identification</title>
<meta name="description" content="page d'identification des utilisateurs">
<script type="text/javascript" charset="utf-8" src="{$bdfUtils->makeUrl('js/lib/jssha256/jssha256.js')}"></script>
<script type="text/javascript" charset="utf-8" src="{$bdfUtils->makeUrl('js/authentication.js')}"></script>

	{include file="inc/header.tpl"}
	<!-- debut du corps de la page -->
  {include file="inc/bdfAuthenticationForm.tpl"}
	<!-- fin du corps de la page -->
	{include file="inc/footer.tpl"}
	{include file="inc/footerHtml.tpl"}




