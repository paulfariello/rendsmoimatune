<?php
/**
 * Fichier de connexion au site via facebook
 *
 * PHP version 5.3
 *
 * This file is part of Rendsmoimatune.
 *
 * Rendsmoimatune is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rendsmoimatune is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rendsmoimatune.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  Rendsmoimatune
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.rendsmoimatune.net
 */

require_once '../inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

try {
    $authentication = \Eu\Rmmt\Authentication\FacebookAuthentication::getAuthentication();
    $authentication->authenticate();
} catch (Exception $e) {
    $te->assign("messages", array(array('type'=>'error', 'content'=>$e->getMessage())));
    $te->assign("_POST", $_POST);
    $te->display("authentication/facebook");
    die();
}

if (null === \Bdf\Session::getInstance()->getCurrentUserId()) {
    $te->assign("messages", array(array('type'=>'error','content'=>\Bdf\Utils::getText('Authentication failed'))));
    $te->assign("_POST", $_POST);
    $te->display("authentication/facebook");
} else {
    $redirect = \Bdf\Session::getInstance()->get('redirect');
    if (null != $redirect) {
        \Bdf\Session::getInstance()->remove('redirect');
        header("location: ".$redirect);
    } else {
        header("location: ".\Bdf\Utils::makeUrl(""));
    }
}
?>
