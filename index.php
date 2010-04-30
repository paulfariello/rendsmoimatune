<?php
  require_once('inc/init.php');
  $em = \Bdf\Core::getInstance()->getEntityManager();
  $em = \Bdf\Core::getInstance()->getEntityManager();

  $_page = \Bdf\Core::getInstance()->getTemplatesEngine();
  $_page->display('index.tpl');
?>
