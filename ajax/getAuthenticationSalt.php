<?php

    $_secu = "all";

    require_once('../inc/init.php');

    $salt = BdfCore::getInstance()->session->getUserSalt($_POST['email']);
    echo json_encode(array("salt"=>$salt));

    BdfCore::getInstance()->logger->setNoDebug();

?>
