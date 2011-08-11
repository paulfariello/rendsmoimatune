<h3>{getText id='Register'}</h3>
<form action="{makeUrl url='register.html'}" class="jNice" method="post" >
    <fieldset>
        <p><label>{getText id='Email'}</label><input type="text" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'email'} user-input-exception{/if}" name="email" value="{if isset($_POST.register)}{$_POST.email}{/if}" /></p>
        <p><label>{getText id='Password'}</label><input type="password" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password" /></p>
        <p><label>{getText id='Password Confirm'}</label><input type="password" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'password'} user-input-exception{/if}" name="password-confirm" /></p>
        <p><label>{getText id='Name'}</label><input type="text" class="text-long {if isset($userInputException) && $userInputException->getInputName() == 'name'} user-input-exception{/if}" name="name" value="{if isset($_POST.name)}{$_POST.name}{/if}"/></p>
        <input type="submit" name="register" value="{getText id='Register'}" />
    </fieldset>
</form>
