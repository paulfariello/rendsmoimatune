<?php
/**
 * Fichier d'initialisation de botteDeFoin
 *
 * PHP version 5.3
 *
 * This file is part of BotteDeFoin.
 *
 * BotteDeFoin is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * BotteDeFoin is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with BotteDeFoin.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

error_reporting(E_ALL | E_STRICT);

define('ROOT', dirname(__FILE__).'/../');
define('INC', ROOT.'inc/');
define('ENGINE', INC.'engine/');
define('COTS', INC.'cots/');
define('CONF', INC.'conf/');

/* includes engine files */
require_once ENGINE.'Bdf/Core.class.php';

// Initialisation
$core = Bdf\Core::getInstance();
$core->initialization();

?>
