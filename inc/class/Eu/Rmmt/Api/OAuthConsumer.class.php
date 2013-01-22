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
 * @version  SVN: 145
 * @link     http://www.Rendsmoimatune.fr
 */

namespace Eu\Rmmt\Api;
use Bdf\Core;
use Bdf\Utils;
use Doctrine\Common\Collections\ArrayCollection;
use Eu\Rmmt\Entity;
use Eu\Rmmt\User;

/**
 * OAuthConsumer
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\User
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class OAuthConsumer extends Entity
{
    private $_id;
    private $_email;
    private $_name;
    private $_url;
    private $_key;
    private $_secret;
    private $_registrationToken;
    private $_tokens;

    /**
     * Create a new api oauth consumer
     *
     * @param string $name public name of this oauth consumer
     * @param string $email email of the creator of this oauth consumer
     * @param string $url url of this oauth consumer (web page, market, etc)
     */
    public function __construct($name, $email, $url = null)
    {
        $this->_name                = $name;
        $this->_email               = $email;
        $this->_url                 = $url;
        $this->_key                 = null;
        $this->_secret              = null;
        $this->_registrationToken   = hash_hmac('SHA1', mt_rand(), $this->_email);
        $this->_tokens              = new ArrayCollection();
    }

    /**
     * Get id
     *
     * @return id
     */
    public function getId()
    {
        return $this->_id;
    }

    /**
     * Get email
     *
     * @return email
     */
    public function getEmail()
    {
        return $this->_email;
    }

    public function getName()
    {
        return $this->_name;
    }

    public function getUrl()
    {
        return $this->_url;
    }

    /**
     * Check registration token
     *
     * @param string $token the token to check
     *
     * @return true if token is valid, false otherwise.
     */
    public function checkRegistrationToken($token)
    {
        return $this->_registrationToken === $token;
    }

    /**
     * Delete token used to check email validity
     */
    public function deleteRegistrationToken()
    {
        $this->_registrationToken = null;
    }

    /**
     * Generate a new API key
     */
    public function generateKey()
    {
        if ($this->_key == null) {
            $this->_key = hash_hmac('SHA1', mt_rand(), $this->_email);
            $this->_secret = hash_hmac('SHA1', mt_rand(), $this->_email);
        }
    }

    /**
     * Get consumer key
     *
     * @return consumer key
     */
    public function getKey()
    {
        return $this->_key;
    }

    /**
     * Get consumer secret
     *
     * @return consumer secret
     */
    public function getSecret()
    {
        return $this->_secret;
    }

    /**
     * Generate a new request token
     *
     * @param string $callback oauth_callback as defined in 2.1. Temporary Credentials
     *
     * @return request token
     */
    public function generateRequestToken($callback)
    {
        $requestToken = new OAuthToken($this, $callback, null);
        $this->_tokens->add($requestToken);

        return $requestToken;
    }

    /**
     * Generate a new access token
     *
     * @param OAuthToken $requestToken request token used to request this access token
     * @param User $user user wich gaves its autorization
     *
     * @return access token
     */
    public function generateAccessToken(OAuthToken $requestToken, User $user)
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $em->getConnection()->beginTransaction();

        if (!$this->_tokens->contains($requestToken))
            return null;

        $accessToken = new OAuthToken($this, $requestToken->getOriginalCallback(), $user);
        $this->_tokens->add($accessToken);

        $accessToken->generateVerifier();
        $requestToken->setVerifier($accessToken->getVerifier());

        $em->flush();
        $em->getConnection()->commit();

        return $accessToken;
    }

    /**
     * Create signature base string for the current request
     *
     * @param OAuth $oauth OAuth object that handle authentication
     *
     * @return string signature base string
     */
    private function _createSignatureBaseString(OAuth $oauth)
    {
        $signatureBaseString = urlencode(strtoupper($_SERVER['REQUEST_METHOD'])).'&';

        $url = 'http'.(empty($_SERVER["HTTPS"])?'':'s').'://'.$_SERVER['SERVER_NAME'].parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
        $signatureBaseString .= urlencode(strtolower($url)).'&';

        //XXX this method doesn't handle duplicate parameters as defined in 3.4.1.3.2. Parameters Normalization
        $args = array();

        $realm = $oauth->getRealm();
        if ($realm != NULL)
            $args['realm'] = $realm;

        $args = array_merge($args, $oauth->getParameters());
        $args = array_merge($args, $oauth->getHTTPAuthorizationParameters());

        if (isset($_SERVER['CONTENT_TYPE']) && $_SERVER['CONTENT_TYPE'] == 'application/x-www-form-urlencoded') {
            $args = array_merge($args, $_POST);
        }

        $args = array_merge($args, $_GET);

        if (isset($args['oauth_signature']))
            unset($args['oauth_signature']);

        ksort($args);
        $signatureBaseString .= urlencode(http_build_query($args, '', '&', PHP_QUERY_RFC3986));

        return $signatureBaseString;
    }

    /**
     * Check signature
     *
     * @param OAuth $oauth OAuth object that handle authentication
     * @param string $signature signature as defined in 3.4. Signature
     * @param OAuthToken $token token
     */
    public function checkSignature(OAuth $oauth, $signature, OAuthToken $token = null)
    {
        $key = urlencode($this->_secret).'&'.urlencode($token == null ? '':$token->getSecret());
        $referenceSignature = hash_hmac('SHA1', $this->_createSignatureBaseString($oauth), $key, true);

        $decodedSignature = base64_decode($signature);

        if ($referenceSignature != $decodedSignature)
            throw new \Eu\Rmmt\Exception\OAuthException(401, "Invalid signature : ".$decodedSignature);
    }

    public function equals(OAuthConsumer $consumer = null)
    {
        if ($consumer == null)
            return false;
        else
            return $this->getKey() === $consumer->getKey();
    }
}
