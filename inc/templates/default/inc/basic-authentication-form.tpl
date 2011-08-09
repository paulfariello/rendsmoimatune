<form action="{makeUrl url='sign-in.html'}" class="jNice" method="post">
    <fieldset>
        <p><label>{getText id='Email'}</label><input type="text" class="text-long" name="email" /></p>
        <p><label>{getText id='Password'}</label><input type="password" class="text-long" name="password" /></p>
        <input type="submit" name="sign-in" value="{getText id='Sign in'}" /><a href="{makeUrl url="password-recovery.html"}" class="button">{getText id="Forgot password ?"}</a>
    </fieldset>
</form>
