</head>

<body>
	<div id="wrapper">
    	<!-- h1 tag stays for the logo, you can use the a tag for linking the index page -->
    	<h1><a href="{makeUrl url=''}"><span>Rends-moi ma tune</span></a></h1>
        
        <!-- You can name the links with lowercase, they will be transformed to uppercase by CSS, we prefered to name them with uppercase to have the same effect with disabled stylesheet -->
        <ul id="mainNav">
        	<li><a href="{makeUrl url='dashboard/'}" class="active">{getText id='DASHBOARD'}</a></li> <!-- Use the "active" class for the active menu item  -->
        	<li><a href="{makeUrl url='events/'}">{getText id='EVENTS'}</a></li>
        	<li><a href="{makeUrl url='my-account/'}">{getText id='MY ACCOUNT'}</a></li>
        	<li class="logout"><a href="{makeUrl url='logout.html'}">{getText id='LOGOUT'}</a></li>
        </ul>
        <!-- // #end mainNav -->
        
        <div id="containerHolder">
			<div id="container">
