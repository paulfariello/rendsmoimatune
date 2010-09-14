</head>

<body>
	<div id="wrapper">
    	<!-- h1 tag stays for the logo, you can use the a tag for linking the index page -->
    	<h1><a href="{makeUrl url=''}"><span>Rends-moi ma tune</span></a></h1>
        
        <!-- You can name the links with lowercase, they will be transformed to uppercase by CSS, we prefered to name them with uppercase to have the same effect with disabled stylesheet -->
        <ul id="mainNav">
        	<li><a href="{makeUrl url='dashboard/'}" {if 'dashboard/'|isCurrentPage}class="active"{/if}>{getText id='DASHBOARD'}</a></li> <!-- Use the "active" class for the active menu item  -->
        	<li><a href="{makeUrl url='events/'}" {if 'events/'|isCurrentPage}class="active"{/if}>{getText id='EVENTS'}</a></li>
        	<li><a href="{makeUrl url='my-account/'}" {if 'my-account/'|isCurrentPage}class="active"{/if}>{getText id='MY ACCOUNT'}</a></li>
        	<li class="logout">{if $currentUser == null}<a href="{makeUrl url='sign-in.html'}" {if 'sign-in.html'|isCurrentPage}class="active"{/if}>{getText id='SIGN IN'}</a>{else}<a href="{makeUrl url='sign-out.html'}">{getText id='SIGN OUT'}</a>{/if}</li>
        </ul>
        <!-- // #end mainNav -->
        
        <div id="containerHolder">
			<div id="container">
