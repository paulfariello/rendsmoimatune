<?php
/**
 * Fichier de classe
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

namespace Eu\Rmmt\Api;
use Bdf\Core;
use Bdf\Utils;
use Doctrine\ORM\EntityManager;
use Eu\Rmmt\User;
use Eu\Rmmt\Exception\OAuthException;

/**
 * Simple implementation of an OAuth server conforming to rfc5849 (i.e. OAuth 1.0A)
 *
 * @category Class
 * @author   Paul Fariello <paul.fariello@gmail.com>
 */
class OAuth
{
    private $_em;
    private $_param;
    private $_httpAuthParam;
    private $_realm;
    private $_consumer;
    private $_user;

    const AUTHORIZATION_HEADER = 0;
    const FORM_ENCODED_BODY = 1;
    const REQUEST_URI_QUERY = 2;

    /**
     * Constructeur
     *
     * @param EntityManager $em Doctrine EntityManager used to manage OAuth objects
     * @param integer $parameterTransmission parameter transmission one of self::AUTHORIZATION_HEADER, self::FORM_ENCODED_BODY, self::REQUEST_URI_QUERY
     *
     * @return OAuth
     */
    public function __construct(EntityManager $em, $parameterTransmission)
    {
        $this->_em          = $em;
        $this->_param       = array();
        $this->_consumer    = NULL;
        $this->_user        = NULL;
        $this->_parseParam($parameterTransmission);
    }

    /**
     * Parse OAuth parameter following one of the locations defined in 3.5. Parameter Transmission
     *
     * @param integer $parameterTransmission parameter transmission one of self::AUTHORIZATION_HEADER, self::FORM_ENCODED_BODY, self::REQUEST_URI_QUERY
     */
    private function _parseParam($parameterTransmission)
    {
        switch($parameterTransmission) {
        case self::AUTHORIZATION_HEADER:
            $this->_parseParamAuthorizationHeader();
            break;
        case self::FORM_ENCODED_BODY:
            $this->_filterParam($_POST);
            break;
        case self::REQUEST_URI_QUERY:
            $this->_filterParam($_GET);
            break;
        }
    }

    /**
     * Parse http authorization following OAuth scheme as defined in 3.5.1. Authorization Header
     *
     */
    private function _parseParamAuthorizationHeader()
    {
        $authorization = NULL;
        if (isset($_SERVER['REMOTE_USER']))
            $authorization = $_SERVER['REMOTE_USER'];
        if (isset($_SERVER['PHP_AUTH_DIGEST']))
            $authorization = $_SERVER['PHP_AUTH_DIGEST'];

        if ($authorization == NULL)
            return;

        $authorization = explode(',', $authorization);
        if (strncmp($authorization[0], 'OAuth', strlen('OAuth')) != 0)
            return;

        if (strncmp($authorization[0], 'OAuth realm=', strlen('OAuth realm=')) == 0)
            $this->_realm = trim(substr($authorization[0], strlen('OAuth realm=')));

        $this->_httpAuthParam= array();
        for ($i = 1; $i < count($authorization); $i++) {
            list($index, $value) = explode('=', trim($authorization[$i]));
            $this->_httpAuthParam[$index] = urldecode($value);
        }

        $this->_filterParam($this->_httpAuthParam);
    }

    /**
     * Filter OAuth param from the yiven array
     *
     * @param Array $array array containing some OAuth param
     */
    private function _filterParam(Array &$array)
    {
        foreach ($array as $index => $value) {
           if (strncmp($index, 'oauth_', strlen('oauth_')) == 0) {
               $this->_param[$index] = $value;
               unset($array[$index]);
           }
        }
    }

    /**
     * Check that required arguments are present
     *
     * @throw OAuthException
     */
    private function _checkArgs($requiredArgs)
    {
        foreach($requiredArgs as $arg) {
            if (!isset($this->_param[$arg])) {
                throw new OAuthException(400, "Missing required parameter : ".$arg);
            }
        }
    }

    /**
     * Check that oauth version, if present, is equal to '1.0'
     *
     * @throw OAuthException
     */
    private function _checkVersion()
    {
        if (isset($this->_param['oauth_version']) && $this->_param['oauth_version'] != '1.0')
            throw new OAuthException(401, "Invalid oauth version : ".$this->_param['oauth_version']);
    }

    /**
     * Check check that signature method used is supported
     *
     * @throw OAuthException
     */
    private function _checkSignatureMethod()
    {
        // We do only support hmac-sha1
        if ($this->_param['oauth_signature_method'] != 'HMAC-SHA1')
            throw new OAuthException(400, "Unsupported signature method ".$this->_param['oauth_signature_method']);
    }

    /**
     * Load consumer
     *
     * @throw OAuthException
     */
    private function _loadConsumer()
    {
        $this->_consumer = OAuthConsumer::getRepository($this->_em)->findOneBy(array('_key' => $this->_param['oauth_consumer_key']));
        if ($this->_consumer == NULL)
            throw new OAuthException(401, "Invalid Consumer Key : ".$this->_param['oauth_consumer_key']);
    }

    /**
     * Get current access token
     *
     * @throw OAuthException
     *
     * @return string access token
     */
    private function _getAccessToken()
    {
        $accessToken = OAuthToken::getRepository($this->_em)->findOneBy(array(
            '_token'=>$this->_param['oauth_token'],
            '_isAccessToken' => true,
            '_consumer' => $this->_consumer->getId(),
        ));

        if ($accessToken == null || !$accessToken->isValid())
            throw new OAuthException(401, "Invalid Access Token : ".$_REQUEST['oauth_token']);

        return $accessToken;
    }

    /**
     * Get current realm
     *
     * @return string realm or NULL if no realm set
     */
    public function getRealm()
    {
        return $this->_realm;
    }

    /**
     * Get OAuth parameters
     *
     * @return Array oauth parameters
     */
    public function getParameters()
    {
        return $this->_param;
    }

    /**
     * Get HTTP Authorization parameters
     *
     * @return Array HTTP authorization parameters
     */
    public function getHTTPAuthorizationParameters()
    {
        return $this->_httpAuthParam;
    }

    /**
     * Check that required arguments are present
     *
     * @throw OAuthException
     */
    public function generateRequestToken()
    {
        $this->_checkArgs(array(
            'oauth_consumer_key',
            'oauth_signature_method',
            'oauth_signature',
            'oauth_timestamp',
            'oauth_nonce',
            'oauth_callback',
        ));

        $this->_checkVersion();
        $this->_checkSignatureMethod();
        $this->_loadConsumer();
        $this->_consumer->checkSignature($this, $this->_param['oauth_signature']);

        $this->_em->getConnection()->beginTransaction();

        $requestToken = $this->_consumer->generateRequestToken($this->_param['oauth_callback']);
        $this->_em->persist($requestToken);

        $this->_em->flush();
        $this->_em->getConnection()->commit();

        return $requestToken;
    }

    public function getRequestToken()
    {
        $requestToken = OAuthToken::getRepository($this->_em)->findOneBy(array(
            '_token'=>$_REQUEST['oauth_token'],
            '_isAccessToken' => false,
        ));

        if ($requestToken == null || !$requestToken->isValid())
            throw new OAuthException(401, "Invalid Request Token : ".$_REQUEST['oauth_token']);

        return $requestToken;
    }

    public function grantAccess(User $user, OAuthToken $requestToken)
    {
        $accessToken = $requestToken->getConsumer()->generateAccessToken($requestToken, $user);
        if ($accessToken == null)
            throw new OAuthException(401, "Invalid Request Token : ".$_REQUEST['oauth_token']);
    }

    public function denyAccess(User $user, OAuthToken $requestToken)
    {
        $this->_em->getConnection()->beginTransaction();

        $this->_em->remove($requestToken);

        $this->_em->flush();
        $this->_em->getConnection()->commit();
    }

    public function getAccessToken()
    {
        $this->_checkArgs(array(
            'oauth_consumer_key',
            'oauth_token',
            'oauth_signature_method',
            'oauth_signature',
            'oauth_timestamp',
            'oauth_nonce',
            'oauth_verifier',
        ));

        $this->_checkVersion();
        $this->_checkSignatureMethod();
        $this->_loadConsumer();

        $requestToken = OAuthToken::getRepository($this->_em)->findOneBy(array(
            '_token' => $this->_param['oauth_token'],
            '_isAccessToken' => false,
            '_consumer' => $this->_consumer->getId(),
        ));

        if ($requestToken == NULL)
            throw new OAuthException(401, "Invalid token : ".$this->_param['oauth_token']);

        if ($requestToken->getVerifier() != $this->_param['oauth_verifier'])
            throw new OAuthException(401, "Invalid verifier : ".$this->_param['oauth_verifier']);

        $this->_consumer->checkSignature($this, $this->_param['oauth_signature'], $requestToken);

        $accessToken = OAuthToken::getRepository()->findOneBy(array(
            '_verifier' => $requestToken->getVerifier(),
            '_isAccessToken' => true,
            '_consumer' => $this->_consumer->getId(),
        ));

        if ($accessToken == NULL)
            throw new OAuthException(401, "Access denied by user");

        $this->_em->getConnection()->beginTransaction();

        $accessToken->setVerifier(NULL);
        $this->_em->remove($requestToken);

        $this->_em->flush();
        $this->_em->getConnection()->commit();

        return $accessToken;
    }

    public function getCurrentUser()
    {
        if ($this->_user == NULL) {
            $this->_checkArgs(array(
                'oauth_consumer_key',
                'oauth_token',
                'oauth_signature_method',
                'oauth_signature',
                'oauth_timestamp',
                'oauth_nonce',
            ));

            $this->_checkVersion();
            $this->_checkSignatureMethod();
            $this->_loadConsumer();
            $accessToken = $this->_getAccessToken();

            $this->_user = $accessToken->getUser();
        }

        return $this->_user;
    }
}
