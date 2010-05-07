<?php

/**
 * Session
 *
 *
 * <p>Singleton de gestion des session, identification etc</p>
 *
 * @name nom
 * @author Needle <paul.fariello@gmail.com>
 * @link
 * @copyright Paul Fariello 2007
 * @version 1.0.0
 * @package Nom du package
 */

namespace Bdf;

class Session {

  /*~*~*~*~*~*~*~*~*~*~*/
  /*  1. propriétés    */
  /*~*~*~*~*~*~*~*~*~*~*/

  /**
   * @var type
   * @desc description
   */

  private static $instance;
  private $values;
  private $userId;
  private $validChallenges = array();
  const FIELD_USER_ID = "bdf-user-id";
  const FIELD_CHALLENGE = "bdf-challenge-id";


  /*~*~*~*~*~*~*~*~*~*~*/
  /*  2. méthodes      */
  /*~*~*~*~*~*~*~*~*~*~*/

  /**
   * Constructeur
   *
   * <p>description</p>
   *
   * @name maClass::__construct()
   * @return void
   */

  private function __construct() {

    $this->values = array();

    session_start();
    // On récupère toutes les variables de session
    $this->values = $_SESSION;
    $_SESSION = array();

    // Récupération des variables particulières
    $this->getValidChallenges();
    $this->userId = isset($this->values[self::FIELD_USER_ID])?$this->values[self::FIELD_USER_ID]:null;
  }

  public static function getInstance() {
    if (!isset(self::$instance)) {
      $c = __CLASS__;
      self::$instance = new $c;
    }

    return self::$instance;
  }

  public function __clone() {
    trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
  }

  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
  /*  2.1 méthodes privées   */
  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/


  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
  /*  2.1 méthodes publiques */
  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/

  /**
   * Nom de la fonction
   *
   * <p>Description de la fonction</p>
   *
   * @name maClass::maFonction()
   * @param void
   * @return void
   */

  public function add($name,$value) {

    if($name != self::FIELD_USER_ID) {
      $this->values[$name] = $value;
      return true;
    } else {
      \Bdf\Core::getInstance()->logger->warn('Vous ne pouvez pas ecraser la variable de session : '.$name,'Session add');
      return false;
    }

  }

  public function remove($name) {
    if(isset($this->value[$name])) {
      unset($this->value[$name]);
    }
  }

  public function get($name) {
    if(isset($this->values[$name])) {
      return $this->values[$name];
    } else {
      return null;
    }
  }

  public function getChallenge() {
    $challenge = $this->getRandomSalt();
    $this->validChallenges[$challenge] = time()+10*60;
    return $challenge;
  }

  private function getRandomSalt($length = 10) {
    return substr(hash('sha256',uniqid(mt_rand(),true)),0,$length);
  }

  private function getValidChallenges() {
    if(isset($this->values[self::FIELD_CHALLENGE])) {
      $this->validChallenges = $this->values[self::FIELD_CHALLENGE];
      unset($this->values[self::FIELD_CHALLENGE]);
    }
    $time = time();
    foreach($this->validChallenges as $challenge=>$timeout) {
      if($timeout < time())
        unset($this->validChallenges[$challenge]);
    }
  }

  public function destroy() {
    $this->values = array();
  }

  public function getCurrentUserId() {
      return $this->userId;
  }

  public function setCurrentUserId($userId){
      $this->userId = $userId;
  }
  
  public function setCurrentUser(Bdf\IUser $user){
      $this->userId = $user->getId();
  }

  public function removeCurrentUser(){
      $this->setCurrentUserId(NULL);
  }

  /**
   * Destructeur
   *
   * <p>Description</p>
   *
   * @name maClass::__destruct()
   * @param void
   * @return void
   */

  public function __destruct() {
    if(!empty($_SESSION)) {
      $_SESSION = array();
    }
    if(!empty($this->validChallenges))
      $this->values[self::FIELD_CHALLENGE] = $this->validChallenges;
      $this->values[self::FIELD_USER_ID] = $this->userId;
    $_SESSION = $this->values;

  }

}

?>
