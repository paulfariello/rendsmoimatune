<h3>{getText id='Register'}</h3>
<form action="{makeUrl url='register.html'}" class="jNice" method="post" >
    <fieldset>
        <p><label>{getText id='Email'}</label><input type="text" class="text-long" name="email" /></p>
        <p><label>{getText id='Password'}</label><input type="password" class="text-long" name="password" /></p>
        <p><label>{getText id='Password Confirm'}</label><input type="password" class="text-long" name="password-confirm" /></p>
        <p><label>{getText id='Name'}</label><input type="text" class="text-long" name="name" /></p>
        <input type="submit" name="register" value="{getText id='Register'}" />
    </fieldset>
</form>
