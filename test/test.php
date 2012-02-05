<?php
include '../inc/init.php';

$message = "If you think toto is right please go to http://www.rendsmoimatune.eu/my-accounts/cadeaux-et-albums-18/accept-merge-11-325ed1823bb2a471ca2da73bfb4d88e4e87022659e3ab3ae68fcff443ba43238.html .";
echo preg_replace("#([[:alpha:]]([[:alnum:]]|-)*[[:alnum:]])|([[:alpha:]])#i", '<a href="$0">$0</a>', 'a toto toto-et-tata.').'<br />';
echo Bdf\Utils::replace_uri($message, '<a href="$0">$0</a>');
?>
