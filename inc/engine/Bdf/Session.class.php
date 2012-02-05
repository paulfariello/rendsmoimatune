<?php

/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of BotteDeFoin.
 *
 * BotteDeFoin is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * BotteDeFoin is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with BotteDeFoin.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

namespace Bdf;

/**
 * Session
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */
class Session
{

    private static $_instance;
    private $_values;
    private $_userId;
    private $_validChallenges = array();
    const FIELD_USER_ID = "bdf-user-id";
    const FIELD_CRASH = "bdf-crash";
    const FIELD_CHALLENGE = "bdf-challenge-id";
    const COOKIE_LIFETIME = 604800;

    /**
     * Constructeur
     *
     * @return Session
     */
    private function __construct()
    {

        $this->_values = array();

    }

    /**
     * Accesseur à l'instance de Session
     *
     * @return Session
     */
    public static function getInstance()
    {
        if (!isset(self::$_instance)) {
            $c = __CLASS__;
            self::$_instance = new $c;
        }

        return self::$_instance;
    }

    /**
     * Redefinition de __clone pour interfire le clonnage de l'instance de Logger
     *
     * @return void
     */
    public function __clone()
    {
        trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
    }

    /**
     * Set cookie parameters.
     * Actually set cookie lifetime, domain and path.
     *
     * @return void
     */
    private function _setCookieParams()
    {
        $path   = parse_url(Core::getInstance()->getConfig('site', 'url'), PHP_URL_PATH);
        $domain = parse_url(Core::getInstance()->getConfig('site', 'url'), PHP_URL_HOST);
        session_set_cookie_params (self::COOKIE_LIFETIME, $path, $domain, false, false);
    }

    /**
     * Session recover.
     * Recover last session. Can be recovered after a crash.
     *
     * @return void
     */
    private function _variableRecover()
    {
        // On récupère toutes les variables de session
        if (isset($_SESSION[self::FIELD_CRASH])) {
            $this->_values = $_SESSION[self::FIELD_CRASH];
        } else {
            $this->_values = $_SESSION;
        }

        // On réinitialize la variable de session
        $_SESSION = array();
        $_SESSION[self::FIELD_CRASH] = $this->_values;

        // Récupération des variables particulières
        $this->_getValidChallenges();

        $this->_userId = isset($this->_values[self::FIELD_USER_ID])?$this->_values[self::FIELD_USER_ID]:null;
    }

    /**
     * Initialization des sessions
     *
     * @return void
     */
    public function initialization()
    {
        $this->_setCookieParams();

        session_start();

        $this->_variableRecover();
    }

    /**
     * Ajoute une variable en session
     *
     * @param string $name  Le nom de la variable
     * @param mixed  $value La variable
     *
     * @return void
     */
    public function add($name,$value)
    {

        if ($name != self::FIELD_USER_ID AND $name != self::FIELD_CRASH) {
            $this->_values[$name] = $value;
            $_SESSION[self::FIELD_CRASH][$name] = $value;
        } else {
            // TODO throw exception
            \Bdf\Core::getInstance()->logger->warn('Vous ne pouvez pas ecraser la variable de session : '.$name, 'Session add');
        }

    }

    /**
     * Supprime une variable de la session
     *
     * @param string $name Le nom de la variable
     *
     * @return void
     */
    public function remove($name)
    {
        if (isset($this->_values[$name])) {
            unset($this->_values[$name]);
        }
    }

    /**
     * Accède à une variable de session
     *
     * @param string $name Le nom de la variable
     *
     * @return mixed
     */
    public function get($name)
    {
        if (isset($this->_values[$name])) {
            return $this->_values[$name];
        } else {
            return null;
        }
    }

    /**
     * Génère un challenge pour une authentification
     *
     * @return string
     */
    public function getChallenge()
    {
        $challenge = $this->_getRandomSalt();
        $this->_validChallenges[$challenge] = time()+10*60;
        return $challenge;
    }

    /**
     * Génère une salt aléatoire
     *
     * @param integer $length La taille du salt
     *
     * @return string
     */
    private function _getRandomSalt($length = 10)
    {
        return substr(hash('sha256', uniqid(mt_rand(), true)), 0, $length);
    }

    /**
     * Accède à la liste des challenges valides
     *
     * @return array
     */
    private function _getValidChallenges()
    {
        if (isset($this->_values[self::FIELD_CHALLENGE])) {
            $this->_validChallenges = $this->_values[self::FIELD_CHALLENGE];
            unset($this->_values[self::FIELD_CHALLENGE]);
        }
        $time = time();
        foreach ($this->_validChallenges as $challenge=>$timeout) {
            if ($timeout < time()) {
                unset($this->_validChallenges[$challenge]);
            }
        }
    }

    /**
     * Supprime toutes les variables de sessions
     *
     * @return void
     */
    public function destroy()
    {
        $this->_values = array();
    }

    /**
     * Accède à l'identifiant de l'utilisateur courant
     *
     * @return mixed
     */
    public function getCurrentUserId()
    {
        return $this->_userId;
    }

    /**
     * Change l'identifiant de l'utilisateur courant
     *
     * @param mixed $userId l'identifiant de l'utilisateur
     *
     * @return void
     */
    public function setCurrentUserId($userId)
    {
        session_regenerate_id(true);
        $_SESSION[self::FIELD_CRASH][self::FIELD_USER_ID] = $userId;
        $this->_userId = $userId;
    }

    /**
     * Change l'utilisateur courant
     *
     * @param IUser $user L'utilisateur
     *
     * @return void
     */
    public function setCurrentUser(Bdf\IUser $user)
    {
        $this->setCurrentUserId($user->getId());
    }

    /**
     * Supprime l'utilisateur courant de la session
     *
     * Peut être utiliser pour deconnecter un utilisateur
     *
     * @return void
     */
    public function removeCurrentUser()
    {
        $this->setCurrentUserId(null);
    }

    /**
     * Destructeur
     *
     * @return void
     */
    public function __destruct()
    {
        if (!empty($_SESSION)) {
            $_SESSION = array();
        }

        if (!empty($this->_validChallenges)) {
            $this->_values[self::FIELD_CHALLENGE] = $this->_validChallenges;
        }

        $this->_values[self::FIELD_USER_ID] = $this->_userId;
        $_SESSION = $this->_values;

    }

}
