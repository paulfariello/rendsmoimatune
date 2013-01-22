<?php
/**
 * API OAuth Consumer registration page
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
 * @link     http://www.rendsmoimatune.eu
 */

require_once '../inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

if (isset($_POST['register-consumer'])) {
    try {
        if (!isset($_POST['name']) || empty($_POST['name']))
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Name is required'), $_POST['name'], 'name');

        if (!isset($_POST['email']) || empty($_POST['email']))
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Email is required'), $_POST['email'], 'email');

        $consumer = new \Eu\Rmmt\Api\OAuthConsumer($_POST['name'], $_POST['email'], $_POST['url']);
        $em->persist($consumer);
        $em->flush();
        $te->assign('consumer', $consumer);
    } catch(Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->assign('userInputException', $e);
    }
} elseif (isset($_GET['registration-token'])) {
    $consumer = \Eu\Rmmt\Api\OAuthConsumer::getRepository()->find($_GET['id']);
    if ($consumer != null AND $consumer->checkRegistrationToken($_GET["registration-token"])) {
        $consumer->deleteRegistrationToken();
        $consumer->generateKey();
        $em->flush();
        $te->assign('consumer', $consumer);
    } else {
        $te->assign("messages", array(array("type"=>"error", "content"=>\Bdf\Utils::getText("Unknown API consumer or token."))));
    }
}

$te->display('api/register-consumer');
?>
