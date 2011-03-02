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

namespace Eu\Rmmt\Authentication;
use Doctrine\Common\Collections\ArrayCollection;
use Bdf\Core;
use Bdf\Utils;
use Eu\Rmmt\User;
use Eu\Rmmt\Exception\MergeException;

/**
 * OAuthentication
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\Authentication
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
abstract class OAuthentication implements IAuthentication
{

    /**
     * State of the authentication
     */
    private $_state;
    const REQUEST_REQUEST_TOKEN_STATE       = 0;
    const REQUEST_ACCESS_TOKEN_STATE        = 1;

    protected $_requestToken;
    protected $_accessToken;
    protected $_expires;

    protected $_user;

    /**
     * Constructeur
     *
     * @return BasicAuthentication
     */
    function __construct()
    {
        $this->_state           = self::REQUEST_REQUEST_TOKEN_STATE;
        $this->_requestToken    = null;
        $this->_accessToken     = null;
        $this->_user            = null;
    }


    public function authenticate()
    {
        // If there is a problem authentication won't be persisted
        $this->_close();
        switch($this->_state) {
            case self::REQUEST_REQUEST_TOKEN_STATE:
                $this->_requestRequestToken();
                $this->_persist();
                $this->_redirectUserToServiceProvider();
                break;
            case self::REQUEST_ACCESS_TOKEN_STATE:
                $this->_requestAccessToken();
                $this->_accessProtectedRessources();
                $this->_setCurrentUser();
                $this->_close();
                break;
        }
    }

    protected function _requestRequestTokent()
    {
        $curl = curl_init($this->_constructRequestRequestTokenUrl());    
        curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
        $rawResult = curl_exec($curl);
        curl_close($curl);

        parse_str($rawResult, $result);
        $this->_expires     = $result['expires'];
        $this->_accessToken = $result['access_token'];
    }

    protected function _redirectUserToServiceProvider()
    {
        $this->_state = self::REQUEST_ACCESS_TOKEN_STATE;
        header('location: '.$this->_constructServiceProviderUrl());
    }

    protected function _requestAccessToken()
    {
        if (isset($_GET['code'])) {
            $this->_requestToken = $_GET['code'];
        }

        if (null != $this->_requestToken) {
            $curl = curl_init($this->_constructRequestAccessTokenUrl());    
            curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
            $rawResult = curl_exec($curl);
            curl_close($curl);

            parse_str($rawResult, $result);
            $this->_expires     = $result['expires'];
            $this->_accessToken = $result['access_token'];
        } else {
            throw new \Exception('Request token is needed in order to request access token');
        } 
    }

    protected function _accessProtectedRessources()
    {
        if (null != $this->_requestToken) {
            $curl = curl_init($this->_constructAccessProtectedRessourcesUrl());    
            curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
            $rawResult = curl_exec($curl);
            curl_close($curl);

            $ressources = json_decode($rawResult, true);
            $this->_handleProtectedRessources($ressources);
        } else {
            throw new \Exception('Access token is needed in order to access protected ressources');
        } 
    }

    protected function _setCurrentUser()
    {
        if (null != $this->_user) {
            \Bdf\Session::getInstance()->setCurrentUserId($this->_user->getId());
            \Bdf\Session::getInstance()->remove('authentication');
        } else {
            throw new \Exception("Error while authenticating");
        }
            
    }

    private function _persist()
    {
        \Bdf\Session::getInstance()->add('authentication', $this);
    }

    private function _close()
    {
        \Bdf\Session::getInstance()->remove('authentication');
    }

    public static function getAuthentication()
    {
        $authentication = \Bdf\Session::getInstance()->get('authentication');
        if ( null == $authentication OR get_class($authentication) != get_called_class()) {
            $authentication = new \Eu\Rmmt\Authentication\FacebookAuthentication();
        }
        return $authentication;
    }

    abstract protected function _constructServiceProviderUrl();
    abstract protected function _constructRequestRequestTokenUrl();
    abstract protected function _constructRequestAccessTokenUrl();
    abstract protected function _constructAccessProtectedRessourcesUrl();
    abstract protected function _handleProtectedRessources(array $ressources);
}
