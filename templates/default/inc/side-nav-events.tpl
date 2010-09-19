        		<div id="sidebar">
                	<ul class="sideNav">
                      {foreach from=$events item='event'}
                    	<li><a href="{$event->getUrlDetail()}" {if isset($currentEvent) and $currentEvent->getId() == $event->getId()}class="active"{/if}>{$event->getName()}</a></li>
                      {/foreach}
                    	<li><a href="{makeUrl url='events/create-new-event.html'}">{getText id='New event'}</a></li>
                    </ul>
                    <!-- // .sideNav -->
                </div>    
                <!-- // #sidebar -->
