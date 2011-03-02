<div id="sidebar">
    <ul class="sideNav">
        {if isset($currentUser)}
            {foreach from=$currentUser->getEvents() item='event'}
                {if isset($currentEvent) and $currentEvent->getId() == $event->getId()}
                    <li class="active">
                        <a href="{$event->getUrlDetail()}">{$event->getName()}</a>
                        <ul class="subNav">
                            <li><a href="{$currentEvent->getUrlExpendituresList()}" class="new">{getText id="Expenditures"}</a></li>
                            <li><a href="{$currentEvent->getUrlRepaymentsList()}" class="new">{getText id="Repayments"}</a></li>
                            <li><a href="{$currentEvent->getUrlCashUp()}" class="cash-up">{getText id="Cash up"}</a></li>
                        </ul>
                    </li>
                {else}
                    <li><a href="{$event->getUrlDetail()}">{$event->getName()|htmlProtect}</a></li>
                {/if}
            {/foreach}
        {/if}
        <li {if 'events/create-new-event.html'|isCurrentPage}class="active"{/if}><a href="{makeUrl url='events/create-new-event.html'}" class="new">{getText id='New event'}</a></li>
    </ul>
<!-- // .sideNav -->
</div>    
<!-- // #sidebar -->
