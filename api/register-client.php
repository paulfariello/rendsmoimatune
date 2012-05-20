<?php
/**
 * Fichier de connexion au site
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

if (isset($_POST['register-client'])) {
    $client = new \Eu\Rmmt\Api\Client($_POST['email']);
    $em->persist($client);
    $em->flush();
    $te->assign('client', $client);
} elseif (isset($_GET['token'])) {
    $client = \Eu\Rmmt\Api\Client::getRepository()->find($_GET['id']);
    if ($client != null AND $client->checkToken($_GET["token"])) {
        $client->deleteToken();
        $client->generateApiKey();
        $em->flush();
        $te->assign('client', $client);
    } else {
        $te->assign("messages", array(array("type"=>"error", "content"=>\Bdf\Utils::getText("Unknown API client or token."))));
    }
}

$te->display('api/register-client');
?>
