<?php
/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of Orgy.
 *
 * Orgy is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Orgy is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Orgy.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  Orgy
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.Rendsmoimatune.fr
 */

namespace Eu\Rmmt\User;
use Doctrine\Common\Collections\ArrayCollection;

/**
 * User
 *
 * @category Class
 * @package  Fr\Orgy\User
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class User implements \Bdf\IUser
{
    private $_id;
    private $_email;
    private $_password;

    /**
     * Constructeur
     *
     * @return User
     */
    function __construct()
    {
    }

    /** {@inheritdoc} */
    public function getSkin()
    {
        return \Bdf\Core::getInstance()->getConfig('site', 'default_skin');
    }

    /** {@inheritdoc} */
    public function getId()
    {
        return $this->_id;
    }

    /** {@inheritdoc} */
    public static function getCurrentUser()
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $idUser = \Bdf\Session::getInstance()->getCurrentUserId();
        $user = $em->getRepository('Fr\Orgy\User\User')->find($idUser);
        return $user;
    }

    /**
     * Authentifie un utilisateur
     *
     * @param string $email    Email
     * @param string $password Clear password
     *
     * @return User
     */
    public static function authenticateUser($email, $password) {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $user = $em->getRepository(__CLASS__)->findOneBy(array('email' => $email));
        if($user !== NULL) {
            if(\Bdf\Utils::comparePassword($password, $user->getPassword()) === TRUE) {
                \Bdf\Session::getInstance()->setCurrentUserId($user->getId());
            }
        }
    }

    /**
     * Getter
     *
     * @return string email
     */
    public function getEmail()
    {
        return $this->_email;
    }

    /**
     * Setter
     *
     * @param string $email email
     *
     * @return void
     */
    public function setEmail($email)
    {
        $this->_email = $email;
    }

    /**
     * Getter
     *
     * @return string hashed password
     */
    public function getPassword()
    {
        return $this->_password;
    }

    /**
     * Setter
     *
     * @param string $password clear password
     *
     * @return void
     */
    public function setPassword($password)
    {
        $this->_password = \Bdf\Utils::hashPassword($password);
    }
}
