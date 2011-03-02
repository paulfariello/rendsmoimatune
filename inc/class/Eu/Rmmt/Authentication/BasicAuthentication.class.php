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
use Eu\Rmmt\Exception\MergeException;
use Eu\Rmmt\User;

/**
 * BasicAuthentication
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\Authentication
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class BasicAuthentication implements IAuthentication
{
    /**
     * State of the authentication
     */
    private $_state;
    private $_email;
    private $_password;
    const GET_CREDENTIALS_STATE     = 0;
    const CHECK_CREDENTIALS_STATE   = 1;

    /**
     * Constructeur
     *
     * @return BasicAuthentication
     */
    function __construct()
    {
        $this->_state = self::GET_CREDENTIALS_STATE;
    }

    /**
     * Authentifie un utilisateur
     *
     * @param string $email    Email
     * @param string $password Clear password
     *
     * @return User
     */
    public function authenticate() 
    {
        switch($this->_state) {
            case self::GET_CREDENTIALS_STATE:
                $this->_displayForm();
                break;
            case self::CHECK_CREDENTIALS_STATE:
                $this->_checkCredentials();
                break;
        }
    }

    private function _checkCredentials()
    {
        $user = User::getRepository()->findOneBy(array('_email' => $this->_email));
        if($user !== NULL) {
            if($user->isRegistered() and Utils::comparePassword($this->_password, $user->getPassword()) === TRUE) {
                \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
            }
        }
    }

    private function _displayForm()
    {
        $this->_state = self::CHECK_CREDENTIALS_STATE;
        header('location: '.Utils::makeUrl('authentication/basic.html'));
    }

    public function setState($state)
    {
        $this->_state = (int)$state;
    }

    public function setEmail($email)
    {
        $this->_email = $email;
    }

    public function setPassword($password)
    {
        $this->_password = $password;
    }
}
