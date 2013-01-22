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

namespace Eu\Rmmt\Exception;
use Exception;
use Eu\Rmmt\Api\Api;

/**
 * ApiException
 *
 * @category Class
 * @package  Eu\Rmmt\Exception
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.rendsmoimatune.eu
 */
class ApiException extends \Exception {
    /**
     * Constructeur
     *
     * @param integer $code error code as defined in Eu\Rmmt\Api\Api.class.php
     * @param Exception $previous parent exception
     *
     * @return UserInputException
     */
    public function __construct($code, Exception $previous)
    {
        http_response_code(Api::getHTTPStatus($code));
        parent::__construct(Api::getErrorMessage($code), $code, $previous);
    }

    /**
     * Is an internal error
     *
     * @return bool true if it is an internal error, false otherwise
     */
    public function isInternal()
    {
        return $this->getCode() == Api::ERROR_INTERNAL;
    }

    /**
     * Get http status
     *
     * @return integer http status as defined in rfc2616 10. Status Code Definitions
     */
    public function getHTTPStatus()
    {
        return Api::getHTTPStatus($code);
    }

    /**
     * Get error full description
     *
     * @return string full error description
     */
    public function getDescription()
    {
        return $this->getPrevious()->getMessage();
    }
}
