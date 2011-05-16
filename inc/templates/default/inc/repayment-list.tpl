<table cellpadding="0" cellspacing="0">
    {foreach from=$repayments item='repayment'}
        <tr>
            <td>{$repayment->getDescription()|htmlProtect}</td>
            <td class="action"><a href="#" class="view">View</a><a href="#" class="edit">Edit</a><a href="#" class="delete">Delete</a></td>
        </tr>
    {foreachelse}
        <tr>
            <td>{getText id="No repayment"}</td>
        </tr>
    {/foreach}
</table>
