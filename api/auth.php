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

$api = new Eu\Rmmt\Api\Api("api.auth");

if (!isset($_REQUEST['type'])) {
    $api->displayError(Eu\Rmmt\Api\Api::ERROR_INVALID_REQUEST, "Missing type argument");
}

switch($_REQUEST['type']) {
case 'plain':
    try {
        $authentication = new \Eu\Rmmt\Authentication\BasicAuthentication();
        $authentication->setState(\Eu\Rmmt\Authentication\BasicAuthentication::CHECK_CREDENTIALS_STATE);
        $authentication->setEmail($_REQUEST['email']);
        $authentication->setPassword($_REQUEST['password']);
        $user = $authentication->authenticate();
    } catch (Exception $e) {
        $api->displayInternalError($e);
        die();
    }
    $currentUser = Eu\Rmmt\User::getCurrentUser();
    if (null === $currentUser) {
        $api->displayError(Eu\Rmmt\Api::ERROR_AUTH_FAIL);
    } else {
        $api->setCurrentUser($currentUser);
        $te->display("token");
    }
    break;
default:
    $te->display("error");
}
?>
