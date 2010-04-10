<?php

    $_secu = "all";

    require_once('inc/init.php');

    BdfCore::getInstance()->session->destroy();

    header('location: identification');

?>
