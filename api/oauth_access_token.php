<?php
/**
 * OAuth access token end-point as defined in RFC 5849 2.3. Token Credentials
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

try {
    $oauth = new Eu\Rmmt\Api\OAuth($em);
    $accessToken = $oauth->getAccessToken();
    echo "oauth_token=".$accessToken->getToken()."&oauth_token_secret=".$accessToken->getSecret()."&oauth_callback_confirmed=true";
} catch (Eu\Rmmt\Exception\OAuthException $e) {
    http_response_code($e->getHttpErrorCode());
    echo $e->getMessage();
}

?>
