<?php
/**
 * Fichier d'inscription après avoir reçu une invitation
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

require_once 'inc/init.php';
require_once 'inc/assignDefaultVar.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

try {
    if (isset($_REQUEST['id'])) {
        $user = Eu\Rmmt\User::getRepository()->find($_REQUEST['id']);
        if (null == $user) {
            throw new Eu\Rmmt\Exception\UnknownUserException($_REQUEST['id']); 
        }
    }
    
    if (!isset($_REQUEST['token']) OR !$user->checkInvitationToken($_REQUEST['token'])) {
        throw new Eu\Rmmt\Exception\InvalidInvitationTokenException($_REQUEST['token']);
    }
} catch(Exception $e) {
    $te->assign('_POST',$_POST);
    $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    $te->display('error');
}

if (isset($_POST['create-new-account-from-invitation'])) {
    try {
        if (!isset($_POST['name']) OR empty($_POST['name'])) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText('Name is required'), $_POST['name'], 'name');
        }

        if (!isset($_POST['email']) OR empty($_POST['email'])) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText('Email is required'), $_POST['email'], 'email');
        }

        if (!isset($_POST['password']) OR empty($_POST['password'])) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText('Password is required'), $_POST['password'], 'password');
        }

        if ($_POST['password'] !== $_POST['password-confirm']) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText('Password are not identicals'), $_POST['password'], 'password');
        }

        $user->setName($_POST['name']);
        $user->setEmail($_POST['email']);
        $user->setPassword($_POST['password']);
        $user->setRegistered(true);

        $em->flush();
        \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
        header('location: '.\Bdf\Utils::makeUrl(''));
    } catch (Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST',$_POST);
        $te->assign('user', $user);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->assign('userInputException', $e);
        $te->display('new-account-from-invitation');
    } catch (Exception $e) {
        $te->assign('_POST',$_POST);
        $te->assign('user', $user);
        $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
        $te->display('new-account-from-invitation');
    }
} else {
    $te->assign('user', $user);
    $te->display('new-account-from-invitation');
}
?>
