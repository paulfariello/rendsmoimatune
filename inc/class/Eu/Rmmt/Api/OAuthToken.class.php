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
use Doctrine\Common\Collections\ArrayCollection;
use Eu\Rmmt\Entity;
use Eu\Rmmt\User;
use DateTime;

/**
 * OAuthToken
 *
 * @category Class
 * @author   Paul Fariello <paul.fariello@gmail.com>
 */
class OAuthToken extends Entity
{
    private $_id;
    private $_consumer;
    private $_user;
    private $_callback;
    private $_token;
    private $_secret;
    private $_verifier;
    private $_creationDate;
    private $_duration;
    private $_isAccessToken;

    const REQUEST_TOKEN = 0;
    const ACCESS_TOKEN  = 1;

    /**
     * Create a new request token
     *
     * @param OAuthConsumer consumer that requested a request token
     * @param string $callback oauth_callback as defined in 2.1. Temporary Credentials
     * @param User $user user wich gaves its autorization
     */
    public function __construct(OAuthConsumer $consumer, $callback, User $user = null)
    {
        if (parse_url($callback) == FALSE)
            new OAuthException(401, 'Invalid callback : '.$callback);

        $this->_consumer        = $consumer;
        $this->_user            = $user;
        $this->_callback        = $callback;
        $this->_token           = hash_hmac('SHA1', mt_rand(), $this->_consumer->getEmail());
        $this->_secret          = hash_hmac('SHA1', mt_rand(), $this->_consumer->getEmail());
        $this->_verifier        = null;
        $this->_creationDate    = new DateTime();
        $this->_duration        = 3600; // 1 hour
        $this->_isAccessToken   = $user == null ? false : true;
    }

    /**
     * Check token validity
     *
     * @return true if token is valid, false otherwise.
     */
    public function isValid()
    {
        if ($this->_duration == 0)
            return true;

        $now = new DateTime();
        return $this->_creationDate->getTimestamp() + $this->_duration > $now->getTimestamp();
    }

    /**
     * Get consumer
     *
     * @return consumer
     */
    public function getConsumer()
    {
        return $this->_consumer;
    }

    /**
     * Get token
     *
     * @return token
     */
    public function getToken()
    {
        return $this->_token;
    }

    /**
     * Get token secret
     *
     * @return token secrect
     */
    public function getSecret()
    {
        return $this->_secret;
    }

    /**
     * Get user
     *
     * @return user
     */
    public function getUser()
    {
        return $this->_user;
    }

    /**
     * Generate a new verifier
     */
    public function generateVerifier()
    {
        return $this->_verifier = hash_hmac('SHA1', mt_rand(), $this->_consumer->getEmail());
    }

    /**
     * Set verifier
     *
     * @param string $verifier verifier as defined in 2.2. Resource Owner Authorization
     */
    public function setVerifier($verifier)
    {
        return $this->_verifier = $verifier;
    }

    /**
     * Get verifier
     *
     * @return verifier as defined in 2.2. Resource Owner Authorization
     */
    public function getVerifier()
    {
        return $this->_verifier;
    }

    /**
     * Get callback url as it was provided by resource consumer
     *
     * @return callback url
     */
    public function getOriginalCallback()
    {
        return $this->_callback;
    }

    /**
     * Get callback url with token and verifier
     *
     * @return callback url
     */
    public function getCallback()
    {
        $query = array();

        $callback = parse_url($this->_callback);
        if (isset($callback['query']))
            parse_str($callback['query'], $query);

        $query['oauth_token'] = $this->_token;
        $query['oauth_verifier'] = $this->_verifier;

        $url = '';

        if (isset($callback['scheme']))
            $url .= $callback['scheme'].'://';
        if (isset($callback['user']))
            $url .= $callback['user'];
        if (isset($callback['pass']))
            $url .= ':'.$callback['pass'];
        if (isset($callback['host'])) {
            if (isset($callback['user']))
                $url .= '@';
            $url .= $callback['host'];
        }
        if (isset($callback['port']))
            $url .= ':'.$callback['port'];
        if (isset($callback['path']))
            $url .= $callback['path'];
        $url .= '?'.http_build_query($query);
        if (isset($callback['fragment']))
            $url .= '#'.$callback['fragment'];

        return $url;
    }

    /**
     * Get token type
     *
     * @return either OAuthToken::REQUEST_TOKEN or OAuthToken::ACCESS_TOKEN
     */
    public function getType()
    {
        return $this->_isAccessToken ? OAuthToken::ACCESS_TOKEN : OAuthToken::REQUEST_TOKEN;
    }

    public function equals(OAuthToken $token = null)
    {
        if ($token == null)
            return false;
        else
            return $this->getConsumer()->equals($token->getConsumer) 
                && $this->getToken() === $token->getToken() 
                && $this->getSecret() === $token->getSecret();
    }
}
