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

namespace Eu\Rmmt;
use Doctrine\Common\Collections\ArrayCollection;
use Bdf\Core;

/**
 * User
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\User
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class User implements \Bdf\IUser
{
    private $_id;
    private $_email;
    private $_password;
    private $_firstName;
    private $_lastName;
    private $_isAdmin    = false;
    private $_registered = true;
    private $_events;
    private $_payers;
    private $_beneficiaries;
    private $_repaymentsFromMe;
    private $_repaymentsToMe;

    /**
     * Constructeur
     *
     * @return User
     */
    function __construct($email)
    {
        $this->_email = $email;
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
        $user = $em->getRepository(__CLASS__)->find($idUser);
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
        $user = $em->getRepository(__CLASS__)->findOneBy(array('_email' => $email));
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

    public function getFirstName()
    {
        return $this->_firstName;
    }

    public function setFirstName($firstName)
    {
        $this->_firstName = $firstName;
    }

    public function getLastName()
    {
        return $this->_lastName;
    }

    public function setLastName($lastName)
    {
        $this->_lastName = $lastName;
    }

    public function getName()
    {
        return $this->getFirstName()." ".$this->getLastName();
    }

    public function isAdmin()
    {
        return $this->_isAdmin;
    }

    public function setIsAdmin($isAdmin)
    {
        $this->_isAdmin = $isAdmin;
    }

    public function getEvents()
    {
        return $this->_events;
    }

    public function addEvent(Event $event)
    {
        $this->_events->add($event);
    }

    public function removeEvent(Event $event)
    {
        $this->_events->removeElement($event);
    }

    public function getPayedExpenditures()
    {
        //TODO get paying users
        return null;
    }

    public function addPayedExpenditure(Expenditure $expenditure, $amount)
    {
        //TODO add paying user
    }

    public function removePayedExpenditure(Expenditure $expenditure)
    {
        //TODO remove paying user
    }

    public function setPayedAmount(Expenditure $expenditure, $amount)
    {
        //TODO set payed amount for given $user
    }

    public function getInvolvedExpenditures()
    {
        //TODO get involved users
        return null;
    }

    public function addInvolvedExpenditure(Expenditure $expenditure, $amount)
    {
        //TODO add involved user
    }

    public function removeInvolvedExpenditure(Expenditure $expenditure)
    {
        //TODO remove involved user
    }

    public function setInvolvedAmount(Expenditure $expenditure, $amount)
    {
        //TODO set involved amount for given $user
    }

    public function getRepaymentsFromMe()
    {
        return $this->_repaymentsFromMe;
    }

    public function addRepaymentsFromMe(Repayment $repaymentsFromMe)
    {
        $this->_repaymentsFromMe->add($repaymentsFromMe);
    }

    public function getRepaymentsToMe()
    {
        return $this->_repaymentsToMe;
    }

    public function addRepaymentsToMe(Repayment $repaymentsToMe)
    {
        $this->_repaymentsToMe->add($repaymentsToMe);
    }

    public function getRegistered() {
        return $this->_registered;
    }

    public function setRegistered($registered) {
        $this->_registered = (boolean)$registered;
    }

    public static function getRepository()
    {
        return Core::getInstance()->getEntityManager()->getRepository(__CLASS__);
    }

    public function getUrlInvite()
    {
        return \Bdf\Utils::makeUrl('user-'.$this->getId().'/invite.html');
    }

}
