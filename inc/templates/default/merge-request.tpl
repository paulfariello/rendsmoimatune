{include file='inc/header-html.tpl' title='Dashboard'}
<!-- Additionnal javascript script -->
{include file='inc/header.tpl'}
{include file='inc/side-nav-my-accounts.tpl'}
<h2><a href="{makeUrl url='merge-requests.html'}">{getText id='Merge requests'}</a></h2>
{include file='inc/main.tpl'}
                    {if $doMerge}
                        <h3>{getText id='Merge request of users %1$s and %2$s' arg1=$mergeRequest->getFirstUser()->getName() arg2=$mergeRequest->getSecondUser()->getName()}</h3>
                        <form action="{$mergeRequest->getUrl()}" method="post" id="merge-user">
                            <input type="hidden" name="merge-id" value="{$mergeRequest->getId()}" />
                            <fieldset>
                                <legend>{getText id="Decinding what informations to keep"}</legend>
                                <div class="first-user">
                                    <input type="radio" name="name" value="1" id="name-u1" {if $mergeRequest->getFirstUser()->isRegistered() OR !$mergeRequest->getSecondUser()->isRegistered()}checked="checked"{/if} /><label for="name-u1">{$mergeRequest->getFirstUser()->getName()}</label>
                                </div>
                                <div class="second-user">
                                    <input type="radio" name="name" value="2" id="name-u2" {if !$mergeRequest->getFirstUser()->isRegistered() AND $mergeRequest->getSecondUser()->isRegistered()}checked="checked"{/if}/><label for="name-u2">{$mergeRequest->getSecondUser()->getName()}</label>
                                </div>
                                {if $mergeRequest->getFirstUser()->isRegistered() AND $mergeRequest->getSecondUser()->isRegistered()}
                                    <div class="first-user">
                                        <input type="radio" name="email" value="1" id="email-u1" checked="checked" /><label for="email-u1">{$mergeRequest->getFirstUser()->getEmail()}</label>
                                    </div>
                                    <div class="second-user">
                                        <input type="radio" name="email" value="2" id="email-u2" checked="checked" /><label for="email-u2">{$mergeRequest->getSecondUser()->getEmail()}</label>
                                    </div>
                                {/if}
                                <input type="submit" name="merge" value="{getText id="Do merge"}" />
                            </fieldset>
                        </form>
                    {/if}
{include file='inc/footer.tpl'}
