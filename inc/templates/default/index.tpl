{include file='inc/header-html.tpl'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-index.tpl'}
{include file='inc/main.tpl'}
    <div class="site-presentation img-left">
        <img src="{makeUrl url='balance.png' type='img'}" />
        <h3>{getText id='Balance of payments'}</h3>
        <p>{getText id='P_Balancing'}</p>
        <div class="clear"></div>
    </div>
    <div class="site-presentation img-right">
        <h3>Rappel automatique</h3>
        <p>{getText id='P_AutomaticRecall'}</p>
        <img src="{makeUrl url='automatic-recall.png' type='img'}" />
        <div class="clear"></div>
    </div>
    <div class="site-presentation img-left">
        <img src="{makeUrl url='user-completion.png' type='img'}" />
        <h3>Pas d'inscription obligatoire</h3>
        <p>{getText id='P_AutomaticUserCreation'}</p>
    </div>
{include file='inc/footer.tpl'}
