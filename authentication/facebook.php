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

$currentUser = Eu\Rmmt\User::getCurrentUser();
if (null === $currentUser) {
    $te->assign("messages", array(array('type'=>'error','content'=>\Bdf\Utils::getText('Authentication failed'))));
    $te->assign("_POST", $_POST);
    $te->display("authentication/facebook");
} else {
    $messages = array();
    $messages[] = array('type'=>'info','content'=>Bdf\Utils::getText('You have successfully been authenticated through Facebook.'));

    $redirect = \Bdf\Session::getInstance()->get('redirect');
    if (null != $redirect) {
        \Bdf\Session::getInstance()->remove('redirect');
        header("location: ".$redirect);
    } elseif ($currentUser->getConnectionCounter() < 2) {
        $messages[] = array('type'=>'info','content'=>Bdf\Utils::getText('Thank you for registering in Rendsmoimatune. You can start with the creation of your first account. An account is a group of expenditures and repayments related by something relevant for you. That thing could be holidays, roommate or even your every day expenditures.'));
        \Bdf\Session::getInstance()->add('messages',$messages);
        header("location: ".\Bdf\Utils::makeUrl("my-accounts/create-new-account.html"));

    } else {
        \Bdf\Session::getInstance()->add('messages',$messages);
        header("location: ".\Bdf\Utils::makeUrl(""));
    }
}
?>
