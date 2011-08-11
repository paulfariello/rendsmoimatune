<?php
/**
 * Fichier d'inscription
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

if (!isset($_POST['register'])) {
    $te->display('register');
} else {
    try {
        if (!isset($_POST['email']) OR empty($_POST['email'])) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Email is required'), '', 'email');
        }

        $user = Eu\Rmmt\User::getRepository()->findOneBy(array('_email'=>$_POST['email']));
        if ($user !== null) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Email %s is already registered, pleasse choose another email.', $_POST['email']), $_POST['email'], 'email');
        }

        if (!isset($_POST['password']) OR empty($_POST['password'])) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Password is required'), '', 'password');
        }

        if ($_POST['password'] !== $_POST['password-confirm']) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Password are not identical'), '', 'password');
        }

        if (!isset($_POST['name']) OR empty($_POST['name'])) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Name is required'), '', 'name');
        }

        $user = new Eu\Rmmt\User($_POST['email']);
        $user->setRegistered(true);
        $user->setPassword($_POST['password']);
        $user->setName($_POST['name']);
        $em->persist($user);
        $em->flush();
        \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
        header('location: '.\Bdf\Utils::makeUrl(''));
    } catch (Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST', $_POST);
        $te->assign('userInputException', $e);
        $te->assign('messages', array(array('type'=>'error', 'content'=>$e->getMessage())));
        $te->display('register');
    } catch (Exception $e) {
        $te->assign('_POST', $_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>\Bdf\Utils::getText('Internal error :').$e->getMessage())));
        $te->display('register');
    }
}
?>
