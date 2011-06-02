{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{makeUrl url='my-accounts/create-new-account.html'}">{getText id='New account'}</a></h2>
{include file='inc/main.tpl'}
                    <div class="site-advice img-left">
                        <img src="{makeUrl url='account-open-medium.png' type='img'}" />
                        <h3>{getText id='Account'}</h3>
                        <p>{getText id='P_AccountAdvice'}</p>
                        <div class="clear"></div>
                    </div>
                	<form action="{makeUrl url='my-accounts/create-new-account.html'}" class="jNice" method="post">
                      <fieldset>
                          <p><label for="name">{getText id='Title'}</label><input type="text" class="text-long{if isset($userInputException) && $userInputException->getInputName() == 'title'} user-input-exception{/if}" name="name" value="{if isset($_POST)}{$_POST.name|htmlProtect}{/if}" /></p>
                          <p><label for="start-date" class="float-left">{getText id='From'}</label><input type="text" id="start-date" class="text-medium date{if isset($userInputException) && $userInputException->getInputName() == 'date'} user-input-exception{/if}" name="start-date" value="{if isset($_POST)}{$_POST.{'start-date'}|htmlProtect}{else}{date('d-m-Y')}{/if}" />
                          <label for="end-date" class="float-left">{getText id='to'}</label><input type="text" id="end-date" class="text-medium date{if isset($userInputException) && $userInputException->getInputName() == 'date'} user-input-exception{/if}" name="end-date" value="{if isset($_POST)}{$_POST.{'end-date'}|htmlProtect}{else}{date('d-m-Y')}{/if}" /></p>
                          <input type="submit" name="create-new-account" value="{getText id='Create'}" />
                      </fieldset>
                  </form>
{include file='inc/footer.tpl'}
