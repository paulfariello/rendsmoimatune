        		<div id="sidebar">
                	<ul class="sideNav">
                      {foreach from=$events item='event'}
                    	<li {if isset($currentEvent) and $currentEvent->getId() == $event->getId()}class="active"{/if}><a href="{$event->getUrlDetail()}">{$event->getName()}</a></li>
                      {/foreach}
                    	<li {if 'events/create-new-event.html'|isCurrentPage}class="active"{/if}><a href="{makeUrl url='events/create-new-event.html'}" class="new">{getText id='New event'}</a></li>
                    </ul>
                    <!-- // .sideNav -->
                </div>    
                <!-- // #sidebar -->
