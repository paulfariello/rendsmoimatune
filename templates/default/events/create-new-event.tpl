{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{makeUrl url='events/create-new-event.html'}">{getText id='New event'}</a></h2>
{include file='inc/main.tpl'}
{include file='inc/message.tpl'}
                	<form action="{makeUrl url='events/create-new-event.html'}" class="jNice" method="post">
                      <fieldset>
                          <p><label for="name">{getText id='Name'}</label><input type="text" class="text-long" name="name" value="{if isset($_POST)}{$_POST.name|htmlspecialchars}{/if}" /></p>
                          <p><label for="start-date" class="float-left">{getText id='From'}</label><input type="text" id="start-date" class="text-medium date" name="start-date" value="{if isset($_POST)}{$_POST.{'start-date'}|htmlspecialchars}{else}{date('m-d-Y')}{/if}" />
                          <label for="end-date" class="float-left">{getText id='to'}</label><input type="text" id="end-date" class="text-medium date" name="end-date" value="{if isset($_POST)}{$_POST.{'end-date'}|htmlspecialchars}{else}{date('m-d-Y')}{/if}" /></p>
                          <input type="submit" name="create-new-event" value="{getText id='Create'}" />
                      </fieldset>
                  </form>
{include file='inc/footer.tpl'}
