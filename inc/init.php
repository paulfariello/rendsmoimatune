<?php

error_reporting(E_ALL | E_STRICT);

define('ROOT',dirname(__FILE__).'/../');
define('INC',ROOT.'inc/');
define('ENGINE',INC.'engine/');
define('COTS',INC.'cots/');
define('CONF',INC.'conf/');

/* includes engine files */
require_once(ENGINE.'Core/Core.class.php');

// Initialisation
$core = Bdf\Core::getInstance();
$core->initialization();

// Configuration de doctrine
// Configuration de smarty


require_once(ENGINE.'Session/Session.class.php');
require_once(ENGINE.'Smarty/Smarty.class.php');
require_once(ENGINE.'Utils/Utils.class.php');
require_once(ENGINE.'User/User.class.php');
// Initialisation du core

// Initialisation des session
//BdfCore::getInstance()->session->setSecurityLevel($_secu);
//BdfCore::getInstance()->session->checkUserHasRight();
//
///* Configuration de smarty */
//$_page = new Smarty();
//$_page->template_dir = ROOT."templates/".BdfCore::getInstance()->session->getUser()->getSkin().'/';
//$_page->compile_dir  = ROOT."templates_c/";
//if(BdfCore::getInstance()->getConfig('logger','level') == 'Bdf::DEBUG') {
//  $_page->debugging = true;
//} else {
//  $_page->debugging = false;
//}
?>
