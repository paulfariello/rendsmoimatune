<?php
/**
 * Fichier corrspondant à la page d'identification
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

require_once 'inc/init.php';

$page = \Bdf\Core::getInstance()->getTemplatesEngine();

if (isset($_POST['bdf-authentication-email'])) {
    if (isset($_POST['bdf-authentication-response']) AND !empty($_POST['bdf-authentication-response'])) {
        BdfCore::getInstance()->session->authentication($_POST['bdf-authentication-email'], $_POST['bdf-authentication-response'], $_POST['bdf-authentication-challenge']);
    } else {
        BdfCore::getInstance()->session->authentication($_POST['bdf-authentication-email'], $_POST['bdf-authentication-password']);
    }
}

$page->assign("challenge", BdfCore::getInstance()->session->getChallenge());

$page->display("identification.tpl");

?>
