<table cellpadding="0" cellspacing="0">
    {foreach from=$repayments item='repayment'}
        <tr>
            <td>{$repayment->getDescription()|htmlProtect}</td>
            <td class="action"><a href="{$repayment->getUrlView()}" class="view">View</a><a href="{$repayment->getUrlEdit()}" class="edit">Edit</a><a href="{$repayment->getUrlDelete()}" class="delete">Delete</a></td>
        </tr>
    {foreachelse}
        <tr>
            <td>{getText id="No repayment"}</td>
        </tr>
    {/foreach}
</table>
