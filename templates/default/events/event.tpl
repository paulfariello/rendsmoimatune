{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/textboxlist.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()}</a></h2>
{include file='inc/main.tpl'}

                	
					<h3>{getText id='Expenditures'}</h3>
                    	<table cellpadding="0" cellspacing="0">
                            {foreach from=$currentEvent->getExpenditures() item='expenditure'}
                            <tr>
                                <td>{$expenditure->getName()}</td>
                                <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
                            </tr>
                            {/foreach}                      
                        </table>
          {include file='events/create-new-expenditure-form.tpl'}
					<h3>{getText id='Repayments'}</h3>
                        <table cellpadding="0" cellspacing="0">
                            {foreach from=$currentEvent->getRepayments() item='repayment'}
                            <tr>
                                <td>{$repayment->getName()}</td>
                                <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
                            </tr>
                            {/foreach}
                        </table>
{include file='inc/footer.tpl'}
