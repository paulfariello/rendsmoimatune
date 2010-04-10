<?php

    $_secu = "all";

    require_once('inc/init.php');

    if(isset($_POST['bdf-authentication-email'])) {
      if(isset($_POST['bdf-authentication-response']) AND !empty($_POST['bdf-authentication-response'])) {
        BdfCore::getInstance()->session->authentication($_POST['bdf-authentication-email'],$_POST['bdf-authentication-response'],$_POST['bdf-authentication-challenge']);
      } else {
        BdfCore::getInstance()->session->authentication($_POST['bdf-authentication-email'],$_POST['bdf-authentication-password']);
      }
    }

    $_page->assign("challenge",BdfCore::getInstance()->session->getChallenge());

    $_page->display("identification.tpl");

?>
