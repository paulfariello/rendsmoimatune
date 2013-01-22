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
use Exception;
use Eu\Rmmt\Api\Api;
use Eu\Rmmt\Exception\OAuthException;
use Eu\Rmmt\Exception\ApiException;
use Eu\Rmmt\User;

/**
 * Api
 *
 * @category Class
 * @author   Paul Fariello <paul.fariello@gmail.com>
 */
class Api
{
    /**
     * List of error code
     */
    const ERROR_INTERNAL            = 0x8000;
    const ERROR_INVALID_REQUEST     = 0x0001;
    const ERROR_AUTH_REQUIRED       = 0x0002;
    const ERROR_ACCESS_FORBIDDEN    = 0x0003;

    private static $ERROR_MESSAGES  = null;
    private static $HTTP_STATUS     = null;

    private $_em;
    private $_te;
    private $_oauth;

    public function __construct(OAuth $oauth)
    {
        $this->_oauth = $oauth;

        $this->_em = Core::getInstance()->getEntityManager();

        $this->_te = Core::getInstance()->getTemplatesEngine();
        $this->_te->setSkin("xml");
        $this->_te->setDebugging(false);
    }

    public function getCurrentUser()
    {
        try {
            return $this->_oauth->getCurrentUser();
        } catch (OAuthException $e) {
            throw new ApiException(Api::ERROR_AUTH_REQUIRED, $e);
        }
    }

    /**
     * Init error messages
     */
    private static function initErrorMessages()
    {
        self::$ERROR_MESSAGES = array(
            self::ERROR_INTERNAL            => 'Internal error',
            self::ERROR_INVALID_REQUEST     => 'Invalid request',
            self::ERROR_AUTH_REQUIRED       => 'Authentication required',
            self::ERROR_ACCESS_FORBIDDEN    => 'Access forbidden',
        );
    }

    /**
     * Translate error code into corresponding string
     *
     * @param integer $errorCode error code
     * @param Exception $errorCode error code
     *
     * @return error message
     */
    public static function getErrorMessage($errorCode)
    {
        if (self::$ERROR_MESSAGES == null)
            self::initErrorMessages();

        if (isset(self::$ERROR_MESSAGES[$errorCode]))
            return self::$ERROR_MESSAGES[$errorCode];
        else
            return 'Unknown error code : '.(int)$errorCode;
    }

    /**
     * Init http error code
     */
    private static function initHTTPStatus()
    {
        self::$HTTP_STATUS = array(
            self::ERROR_INTERNAL            => 500,
            self::ERROR_INVALID_REQUEST     => 400,
            self::ERROR_AUTH_REQUIRED       => 401,
            self::ERROR_ACCESS_FORBIDDEN    => 401,
        );
    }

    /**
     * Translate error code into corresponding http status
     *
     * @param integer $errorCode error code
     *
     * @return integer http status as defined in rfc2616 10. Status Code Definitions
     */
    public static function getHTTPStatus($errorCode)
    {
        if (self::$HTTP_STATUS == null)
            self::initHTTPStatus();

        if (isset(self::$HTTP_STATUS[$errorCode]))
            return self::$HTTP_STATUS[$errorCode];
        else
            return 500;
    }
}
