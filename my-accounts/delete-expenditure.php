<?php
/**
 * Fichier de suppression d'une nouvelle dépense
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
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.rendsmoimatune.eu
 */

require_once '../inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

$currentUser = \Eu\Rmmt\User::getCurrentUser();
if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}

if (!isset($_GET['expenditure-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
} else {
    $expenditure = Eu\Rmmt\Expenditure::getRepository()->find($_GET['expenditure-id']);
    if ($expenditure === null) {
        header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    } else {
        try {
            $url = $expenditure->getAccount()->getUrlDetail();

            $expenditure->checkDeleteRight($currentUser);

            $em->remove($expenditure);
            $em->flush();

            $messages = array();
            $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('Expenditure deleted'));
            \Bdf\Session::getInstance()->add('messages',$messages);
            header('location: '.$url);
        } catch(Eu\Rmmt\Exception\RightException $e) {
            \Bdf\Session::getInstance()->add('messages', array(array('type'=>'error','content'=>$e->getMessage())));
            header('location: '.$url);
        }
    }
}

?>
