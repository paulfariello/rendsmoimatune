<form action="{makeUrl url='sign-in.html'}" class="jNice" method="post">
    <fieldset>
        <p><label>{getText id='Email'}</label><input type="text" class="text-long" name="email" /></p>
        <p><label>{getText id='Password'}</label><input type="password" class="text-long" name="password" /></p>
        <input type="submit" name="sign-in" value="{getText id='Sign in'}" />
    </fieldset>
</form>
<a href="{makeUrl url='authentication/facebook.html'}" ><img src="{makeUrl url='facebook-login-button.png' type='img'}" alt="Login with faceebook" /></a>
