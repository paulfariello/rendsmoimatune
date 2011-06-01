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

if (!isset($_POST['create-new-account'])) {
    $te->display('new-account');
} else {
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
        $user->setRegistered(true);
        $user->setPassword($_POST['password']);
        $user->setName($_POST['name']);
        $em->persist($user);
        $em->flush();
        \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
        header('location: '.\Bdf\Utils::makeUrl(''));
    }
}
?>
