window.addEvent("domready", function()
{
  var salt = null;
  if($chk($('bdf-authentication'))) {
    $('bdf-authentication-email').addEvent('blur',function()
    {
      new Request.JSON(
      {
        url: "ajax/getAuthenticationSalt.php",
        data: 'email='+this.getProperty('value'),
        onSuccess: function(response)
        {
          salt = response.salt;
        }
      }).send();
    });

    $('bdf-authentication').addEvent('submit',function(event)
    {
      if(salt != null) {
        var password = $('bdf-authentication-password').getProperty('value');
        var challenge = $('bdf-authentication-challenge').getProperty('value');
        var response = HMAC_SHA256_MAC(challenge,HMAC_SHA256_MAC(salt,password));
        $('bdf-authentication-response').setProperty('value','{SHA256}'+response);
        $('bdf-authentication-password').setProperty('value','');
      }
    });
  }
});
