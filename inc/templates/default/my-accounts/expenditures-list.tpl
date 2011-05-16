{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/js-includes/date-picker.tpl'}
{include file='inc/js-includes/manage-expenditure-users.tpl'}
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='my-accounts/'}">{getText id='My accounts'}</a> &raquo; <a href="{$currentAccount->getUrlDetail()}">{$currentAccount->getName()|htmlProtect}</a> &raquo; <a href="{$currentAccount->getUrlExpendituresList()}">{getText id="Expenditures"}</a></h2>
{include file='inc/main.tpl'}

                	
<h3>{getText id='Expenditures'}</h3>
    {include file="inc/expenditure-list.tpl" expenditures=$expenditures}
    <a href="{$currentAccount->getUrlNewExpenditure()}" class="button add">{getText id="Add"}</a>
    <div class="pagination">
        <div class="newer">
            <a href="{$currentAccount->getUrlExpendituresList(1)}" class="newest{if $page < 2} hidden{/if}">{getText id="Newest"}</a><a href="{$currentAccount->getUrlExpendituresList($page-1)}" class="newer{if $page < 3} hidden{/if}">{getText id="Newer"}</a>
        </div>
        <div class="older">
            <a href="{$currentAccount->getUrlExpendituresList($page+1)}" class="older{if $page > ($lastPage - 2)} hidden{/if}">{getText id="Older"}</a><a href="{$currentAccount->getUrlExpendituresList($lastPage)}" class="oldest{if $page > ($lastPage - 1)} hidden{/if}">{getText id="Oldest"}</a>
        </div>
        <span class="current">{getText id="page %d" arg1=$page}</span>
    </div>
{include file='inc/footer.tpl'}
