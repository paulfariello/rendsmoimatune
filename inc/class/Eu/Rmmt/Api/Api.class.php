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
use Exception;
use Bdf\Core;
use Eu\Rmmt\User;

/**
 * Api
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\User
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class Api
{
    /**
     * List of error code
     */
    const ERROR_INTERNAL            = 0x8000;
    const ERROR_INVALID_REQUEST     = 0x0001;
    const ERROR_INVALID_API_KEY     = 0x0002;
    const ERROR_INVALID_AUTH_TOKEN  = 0x0003;

    /**
     * List of query key
     */
    const QUERY_KEY_API_KEY     = "api-key";
    const QUERY_KEY_AUTH_TOKEN  = "auth-token";

    private $_em;
    private $_te;
    private $_method;
    private $_client = null;
    private $_user;

    public function __construct($method)
    {
        $this->_method = $method;

        $this->_em = Core::getInstance()->getEntityManager();

        $this->_te = Core::getInstance()->getTemplatesEngine();
        $this->_te->setSkin("xml");
        $this->_te->assign("method", $this->_method);

        $this->_checkApiKey();
    }

    private function _checkApiKey()
    {
        if (isset($_REQUEST[self::QUERY_KEY_API_KEY])) {
            $apiClient = Client::getRepository()->findOneBy(array("_apiKey"=>$_REQUEST[self::QUERY_KEY_API_KEY]));
            if ($apiClient != null) {
                $this->_client = $apiClient;
                return;
            }
        }

        $this->displayError(self::ERROR_INVALID_API_KEY);
    }

    public function checkAuthToken()
    {
        if (isset($_REQUEST[self::QUERY_KEY_AUTH_TOKEN])) {
            $query = $em->createQuery('SELECT u FROM Eu\Rmmt\User u INNER JOIN u._apiClient c WHERE u._apiAuthToken = :apiAuthToken AND c._apiKey = :apiKey;');
            $query->setParameters(array("apiAuthToken"=>$_REQUEST[self::QUERY_KEY_AUTH_TOKEN], "apiKey"=>$this->_client->getApiKey()));
            $query->execute();
            $user = User::getRepository()->findOneBy(array("_apiKey"=>$this->_client->getApiKey(), "_apiAuthToken"=>$_REQUEST[self::QUERY_KEY_AUTH_TOKEN]));
            if ($user != null) {
                $this->_user = $user;
                $this->_renewAuthToken();
                return;
            }
        }

        $this->displayError(self::ERROR_INVALID_AUTH_TOKEN);
    }

    private function _generateToken()
    {
        return hash_hmac('SHA512', mt_rand(), $this->_user->getId());
    }

    private function _renewAuthToken()
    {
        $this->_user->setApiAuthToken($this->_generateToken(), $this->_client);
        // TODO be sure to only flush user authToken update
        $this->_em->flush();

        $this->_te->assign("authToken", $this->_user->getApiAuthToken());
    }

    public function setCurrentUser(User $user)
    {
        $this->_user = $user;
        $this->_renewAuthToken();
    }

    public function getCurrentUser()
    {
        return $this->_user;
    }
    
    public function displayInternalError(\Exception $e)
    {
        $this->displayError(self::ERROR_INTERNAL);
    }

    public function displayError($code, $desc = null)
    {
        $this->_te->assign("errorCode", $code);
        if ($desc != null) {
            $this->_te->assign("errorDesc", $desc);
        }
        $this->_te->display("error");
        die();
    }
}
