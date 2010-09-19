{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-events.tpl'}
<h2><a href="{makeUrl url='events/'}">{getText id='Events'}</a> &raquo; <a href="{$currentEvent->getUrlDetail()}">{$currentEvent->getName()}</a></h2>
{include file='inc/main.tpl'}

                	<form action="" class="jNice">
					<h3>{getText id='Expenditures'}</h3>
                    	<table cellpadding="0" cellspacing="0">
                            {foreach from=$currentEvent->getExpenditures() item='expenditure'}
                            <tr>
                                <td>{$expenditure->getName()}</td>
                                <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
                            </tr>
                            {/foreach}                      
                        </table>
					<h3>{getText id='repayments'}</h3>
                        <table cellpadding="0" cellspacing="0">
                            {foreach from=$currentEvent->getRepayments() item='repayment'}
                            <tr>
                                <td>{$repayment->getName()}</td>
                                <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
                            </tr>
                            {/foreach}
                        </table>
					<h3>{getText id='New expenditure'}</h3>
                    	<fieldset>
                        	<p><label>Sample label:</label><input type="text" class="text-long" /></p>
                        	<p><label>Sample label:</label><input type="text" class="text-medium" /><input type="text" class="text-small" /><input type="text" class="text-small" /></p>
                            <p><label>Sample label:</label>
                            <select>
                            	<option>Select one</option>
                            	<option>Select two</option>
                            	<option>Select tree</option>
                            	<option>Select one</option>
                            	<option>Select two</option>
                            	<option>Select tree</option>
                            </select>
                            </p>
                        	<p><label>Sample label:</label><textarea rows="1" cols="1"></textarea></p>
                            <input type="submit" value="Submit Query" />
                        </fieldset>
                    </form>
{include file='inc/footer.tpl'}
