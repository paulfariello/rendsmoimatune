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

if (isset($_POST['create-new-account-from-invitation'])) {
    $doSave = true;
    if ($_POST['password'] !== $_POST['password-confirm']) {
        $doSave = false;
        $te->assign('message', array('type'=>'error','content'=>\Bdf\Utils::getText('Password are not identical')));
        $te->display('new-account');
    }

    if (!isset($_POST['email']) OR empty($_POST['email'])) {
        $doSave = false;
        $te->assign('message', array('type'=>'error','content'=>\Bdf\Utils::getText('Email is required')));
        $te->display('new-account');
    }

    if ($doSave) {
        $user = new Eu\Rmmt\User($_POST['email']);
        $user->setPassword($_POST['password']);
        $user->setFirstName($_POST['first-name']);
        $user->setLastName($_POST['last-name']);
        $em->persist($user);
        $em->flush();
        \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
        header('location: '.\Bdf\Utils::makeUrl(''));
    }
} else {
    try {
        if (isset($_GET['id'])) {
            $user = Eu\Rmmt\User::getRepository()->find($_GET['id']);
            if (null == $user) {
                throw new Eu\Rmmt\Exception\UnkownUserException($_GET['id']); 
            }
        }
        
        if (!isset($_GET['token']) OR !$user->checkInvitationToken($_GET['token'])) {
            throw new Eu\Rmmt\Exception\InvalidInvitationToken();
        }

        $te->assign('user', $user);
        $te->display('new-account-from-invitation');
    } catch(Exception $e) {
        $te->assign('currentEvent',$event);
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->display('error');
    }
}
?>
