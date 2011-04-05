<?php
include '../inc/init.php';

echo Bdf\Utils::ngetText('User %1$s has been created. <a href="%2$s">Invite him ?</a>', 'Users %1$s has been created. <a href="%2$s">Invite them ?</a>', 2, 'paul fariello, jean rodiere', 'http://localhost:8888/Rendsmoimatune/my-parameters/send-invitation.html');
?>
