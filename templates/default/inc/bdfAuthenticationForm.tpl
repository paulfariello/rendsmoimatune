<!-- Authentication form -->
  <form method="post" action="{$bdfUtils->makeUrl('identification')}" id="bdf-authentication">
    <input type="text" name="bdf-authentication-email" id="bdf-authentication-email"/>
    <input type="text" name="bdf-authentication-password" id="bdf-authentication-password"/>
    <input type="hidden" name="bdf-authentication-challenge" value="{$challenge}" id="bdf-authentication-challenge"/>
    <input type="hidden" name="bdf-authentication-response" value="" id="bdf-authentication-response"/>
    <input type="hidden" name="bdf-authentication-salt" value="" id="bdf-authentication-salt"/>
    <input type="hidden" name="bdf-authentication-redirection" value="{$redirection}" id="bdf-authentication-redirection">
    <input type="submit"/>
  </form>
