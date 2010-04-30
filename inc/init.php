<?php

error_reporting(E_ALL | E_STRICT);

define('ROOT',dirname(__FILE__).'/../');
define('INC',ROOT.'inc/');
define('ENGINE',INC.'engine/');
define('COTS',INC.'cots/');
define('CONF',INC.'conf/');

/* includes engine files */
require_once(ENGINE.'Bdf/Core.class.php');

// Initialisation
$core = Bdf\Core::getInstance();
$core->initialization();

?>
