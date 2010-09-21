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
          <h3>{getText id='New expenditure'}</h3>
            <form action="" class="jNice">
                <fieldset>
                  <p><label>{getText id='Name'} :</label><input type="text" class="text-long" name="name" /></p>
                  <p><label>{getText id='Date'} :</label><input type="text" class="text-medium date" name="date" /></p>
                  <p><label>{getText id='Amount'} :</label><input type="text" class="text-medium" name="amount" /></p>
                  <p><label>{getText id='Payers'} :</label><input type="text" class="text-long textboxuserlist" name="payed" /></p>
                  <p><label>{getText id='Involved users'} :</label><input type="text" class="text-long textboxuserlist" name="involved" /></p>
                  <input type="submit" value="Add expenditure" />
                </fieldset>
            </form>
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
