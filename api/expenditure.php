<?php
/**
 * Get an expenditure
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
 * @category ScriptFile
 * @package  Rendsmoimatune
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.rendsmoimatune.eu
 */

require_once '../inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

$api = new Eu\Rmmt\Api\Api(new Eu\Rmmt\Api\OAuth($em));

try {
    $currentUser = $api->getCurrentUser();

    $expenditure = Eu\Rmmt\Expenditure::getRepository()->find($_GET['id']);
    if (NULL == $expenditure) {
        throw new Eu\Rmmt\Exception\ApiException(Eu\Rmmt\Api\Api::UNKNOW_ACCOUNT);
    }

    $expenditure->checkViewRight($currentUser);

    $te->assign("expenditure", $expenditure);
    $te->display("expenditure");
} catch(Eu\Rmmt\Exception\ApiException $e) {
    $te->assign("apiException", $e);
    $te->display("error");
} catch(Eu\Rmmt\Exception\RightException $e) {
    $apiException = new Eu\Rmmt\Exception\ApiException(Eu\Rmmt\Api\Api::ERROR_ACCESS_FORBIDDEN, $e);
    $te->assign("apiException", $apiException);
    $te->display("error");
} catch(Exception $e) {
    $apiException = new Eu\Rmmt\Exception\ApiException(Eu\Rmmt\Api\Api::ERROR_INTERNAL, $e);
    $te->assign("apiException", $apiException);
    $te->display("error");
}
?>
