</head>

<body>
	<div id="wrapper">
    	<!-- h1 tag stays for the logo, you can use the a tag for linking the index page -->
    	<h1><a href="{makeUrl url=''}"><span>Rends-moi ma tune</span></a></h1>
        
        <!-- You can name the links with lowercase, they will be transformed to uppercase by CSS, we prefered to name them with uppercase to have the same effect with disabled stylesheet -->
        <ul id="mainNav">
        	<li {if 'dashboard/'|isCurrentPage}class="active"{/if}><a class="home" href="{makeUrl url='dashboard/'}" >{getText id='DASHBOARD'}</a></li> <!-- Use the "active" class for the active menu item  -->
        	<li {if 'my-accounts/'|isCurrentPage}class="active"{/if}><a class="my-accounts" href="{makeUrl url='my-accounts/'}">{getText id='MY ACCOUNTS'}</a></li>
        	{* <li {if 'my-parameters/'|isCurrentPage}class="active"{/if}><a class="my-parameters" href="{makeUrl url='my-parameters/'}">{getText id='MY PARAMETERS'}</a></li> *}
            {if $currentUser == null}
                <li class="logout"><a href="{makeUrl url='sign-in.html'}" {if 'sign-in.html'|isCurrentPage}class="active"{/if}>{getText id='SIGN IN'}</a></li>
            {else}
                <li class="logout"><a class="sign-out" href="{makeUrl url='sign-out.html'}">{getText id='SIGN OUT'}</a></li>
                <li {if 'my-parameters/'|isCurrentPage}class="active"{/if} class="current-user"><a href="{makeUrl url='my-parameters/'}">{$currentUser->getName()}</a></li>
            {/if}
        </ul>
        <!-- // #end mainNav -->
        
        <div id="containerHolder">
			<div id="container">
