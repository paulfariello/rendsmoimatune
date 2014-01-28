<?php
/**
 * Sign the current http request
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

$requiredArgs = array(
    'oauth_consumer_key',
    'oauth_signature_method',
    'oauth_timestamp',
    'oauth_nonce',
);

try {
    $authorization = NULL;
    if (isset($_SERVER['REMOTE_USER']))
        $authorization = $_SERVER['REMOTE_USER'];
    if (isset($_SERVER['PHP_AUTH_DIGEST']))
        $authorization = $_SERVER['PHP_AUTH_DIGEST'];

    $authorization = explode(',', $authorization);

    $realm = NULL;
    if (strncmp($authorization[0], 'OAuth realm=', strlen('OAuth realm=')) == 0)
        $realm = trim(substr($authorization[0], strlen('OAuth realm=')));

    $oauthParam= array();
    for ($i = 1; $i < count($authorization); $i++) {
        list($index, $value) = explode('=', trim($authorization[$i]));
        $oauthParam[$index] = urldecode($value);
    }

    foreach($requiredArgs as $arg) {
        if (!isset($oauthParam[$arg])) {
            throw new Eu\Rmmt\Exception\OAuthException(400, "Missing required parameter : ".$arg);
        }
    }

    $consumer = Eu\Rmmt\Api\OAuthConsumer::getRepository()->findOneBy(array("_key" => $oauthParam['oauth_consumer_key']));
    if ($consumer == NULL)
        throw new Eu\Rmmt\Exception\OAuthException(401, "Invalid Consumer Key : ".$oauthParam['oauth_consumer_key']);

    // We do only support hmac-sha1
    if ($oauthParam['oauth_signature_method'] != "HMAC-SHA1")
        throw new Eu\Rmmt\Exception\OAuthException(400, "Unsupported signature method ".$oauthParam['oauth_signature_method']);


    if (isset($oauthParam['oauth_token'])) {
        $token = Eu\Rmmt\Api\OAuthToken::getRepository()->findOneBy(array("_token" => $oauthParam['oauth_token']));
        if ($consumer == NULL)
            throw new Eu\Rmmt\Exception\OAuthException(401, "Invalid token : ".$oauthParam['oauth_token']);
    } else {
        $token = null;
    }

    $signatureBaseString = urlencode(strtoupper($_SERVER['REQUEST_METHOD'])).'&';

    $url = $oauthParam['oauth_endpoint'];
    $signatureBaseString .= urlencode(strtolower($url)).'&';

    //XXX this method doesn't handle duplicate parameters as defined in 3.4.1.3.2. Parameters Normalization
    $args = array();

    if ($realm != NULL)
        $args['realm'] = $realm;
    $args = array_merge($args, $oauthParam);

    if (isset($_SERVER['CONTENT_TYPE']) && $_SERVER['CONTENT_TYPE'] == 'application/x-www-form-urlencoded') {
        $args = array_merge($args, $_POST);
    }

    $args = array_merge($args, $_GET);

    if (isset($args['oauth_signature']))
        unset($args['oauth_signature']);
    if (isset($args['oauth_endpoint']))
        unset($args['oauth_endpoint']);

    ksort($args);
    $signatureBaseString .= urlencode(http_build_query($args, '', '&', PHP_QUERY_RFC3986));

    $key = urlencode($consumer->getSecret()).'&'.urlencode($token == null ? '':$token->getSecret());
    $referenceSignature = hash_hmac('SHA1', $signatureBaseString, $key, true);
    $args['oauth_signature'] = base64_encode($referenceSignature);

    unset($args['realm']);
    echo "curl -s -H 'Authorization: OAuth ".($realm == NULL?"":("realm=".urlencode($realm).", ")).http_build_query($args, '', ', ', PHP_QUERY_RFC3986)."' ".$oauthParam['oauth_endpoint']." | xmllint --schema rmmt.xsd -\n";
} catch (Eu\Rmmt\Exception\OAuthException $e) {
    http_response_code($e->getHttpErrorCode());
    echo $e->getMessage();
}

?>
