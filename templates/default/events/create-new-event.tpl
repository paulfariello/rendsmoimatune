{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{makeUrl url='events/create-new-event.html'}">{getText id='New event'}</a></h2>
{include file='inc/main.tpl'}

                	<form action="{makeUrl url='events/create-new-event.html'}" class="jNice" method="post">
                      <fieldset>
                          <p><label for="name">{getText id='Name'}</label><input type="text" class="text-long" name="name" /></p>
                          <p><label for="start-date">{getText id='Start date'}</label><input type="text" class="text-long date" name="start-date" /></p>
                          <p><label for="end-date">{getText id='End date'}</label><input type="text" class="text-long date" name="end-date" /></p>
                      </fieldset>
                  </form>
{include file='inc/footer.tpl'}
