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

class BdfSession {

  /*~*~*~*~*~*~*~*~*~*~*/
  /*  1. propriétés    */
  /*~*~*~*~*~*~*~*~*~*~*/

  /**
   * @var type
   * @desc description
   */

  private static $instance;
  private $securityLevel;
  private $values;
  private $user;
  private $validChallenges;

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

    $this->securityLevel = 'all';
    $this->validChallenges = array();
    $this->user = null;
    $this->values = array();

    session_start();
    // On récupère toutes les variables de session
    $this->values = $_SESSION;
    $_SESSION = array();
    $this->getValidChallenges(); 
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

  public function add($nom,$value) {

    if($nom != 'bdfUser') {
      $this->values[$nom] = $value;
      return true;
    } else {
      BdfCore::getInstance()->logger->warn('Vous ne pouvez pas ecraser la variable de session : bdfUser','Session add');
      return false;
    }

  }


  public function get($nom) {
    if(isset($this->values[$nom])) {
      return $this->values[$nom];
    } else {
      BdfCore::getInstance()->logger->warn('Valeur inexistante : '.$nom,'Session');
      return false;
    }
  }

  public function setSecurityLevel($level) {
    $core = BdfCore::getInstance();
    // Est-ce que le niveau de sécurité requis existe en base de donnée
    $res = $core->db->query("SELECT id FROM ".$core->getConfig('sgbd','prefix')."_Security WHERE nom = '".$core->utils->escapeStr($level)."' LIMIT 1");
    if($core->db->numRows($res) > 0) {
      $this->securityLevel = $level;  
    } else {
      $core->logger->warn('Le niveau de securité n\'existe pas','Security Level');
    }
  }

  public function checkUserHasRight() {
    $userRight = $this->getUserRight();
    if(!in_array($this->securityLevel,$userRight)) {
      if($this->getUserId() !== 0) {
        header('location: '.BdfCore::getInstance()->utils->makeUrl('acces-interdit'));
      } else {
        header('location: '.BdfCore::getInstance()->utils->makeUrl('identification'));
      }
      die();
    }
    BdfCore::getInstance()->logger->info($this->getUserId(),'User Id');
  }


  private function getUserRight() {

    $listeDroits = $this->getListeDroit();
    $droits = array('all');
    $userId = $this->getUserId();
    $core = BdfCore::getInstance();

    if($userId === 0) // Si l'utilisateur n'est pas connecté
      return $droits;


    // On récupère l'id du droit de l'utilisateur
    $res = $core->db->query("SELECT ".$core->getConfig('sgbd','prefix')."_User_Right.id_right FROM ".$core->getConfig('sgbd','prefix')."_User_Right WHERE ".$core->getConfig('sgbd','prefix')."_User_Right.id_user = '".$this->getUserId()."'");
    while($idDroitUser = $core->db->fetchArray($res)) {
      $idDroitUser = $idDroitUser['id_right'];

      $temp = $listeDroits[$idDroitUser];

      while($temp['parent'] != 0) {
        $droits[] = $temp['nom'];
        $temp = $listeDroits[$temp['parent']];
      }
    }

    $core->logger->info($droits,"Droits de l'utilisateur");

    return $droits;

  }

  private function getListeDroit() {
    $core = BdfCore::getInstance();

    $listeDroit = array();
    $res = $core->db->query("SELECT ".$core->getConfig('sgbd','prefix')."_Security.* FROM ".$core->getConfig('sgbd','prefix')."_Security");
    while($result = $core->db->fetchArray($res)) {
      $listeDroit[$result['id']] = $result;
    }

    return $listeDroit;
  }
  
  private function getUserId() {
    $userId = 0;
    if(isset($this->values['BdfUser']) AND is_numeric($this->values['BdfUser'])) {
      $userId = $this->values['BdfUser'];
    }
    return $userId;
  }

  private function setUserId($id) {
    $id = (int)$id;

    $this->values['BdfUser'] = $id;
  }

  public function userHasRight() {
    return (bool)in_array($right,$this->getUserRight());
  }

  public function getUser() {
    if($user === NULL)
      $this->user = new User($this->getUserId());
    return $this->user;
  }

  public function getChallenge() {
    $challenge = $this->getRandomSalt();
    $this->validChallenges[$challenge] = time()+10*60;
    return $challenge;
  }

  private function getRandomSalt($length = 10) {
    return substr(hash('sha256',uniqid(mt_rand(),true)),0,$length);
  }

  public function authentication($email,$password,$challenge = null) {
    $core = BdfCore::getInstance();
    $auth = false;
    $id = 0;

    // On récupète le mot de passe
    $res = $core->db->query("
      SELECT 
        ".$core->getConfig('sgbd','prefix')."_User.id,
        ".$core->getConfig('sgbd','prefix')."_User.password
      FROM
        ".$core->getConfig('sgbd','prefix')."_User
      WHERE
        ".$core->getConfig('sgbd','prefix')."_User.email = '".$core->utils->escapeStr($email)."'
      LIMIT 1");

    if($core->db->numRows($res) > 0) {
      list($id,$goodPassword) = $core->db->fetchRow($res);

      // On récupère le type de hash du bon password
      preg_match('#^{([a-z0-9]+)}([a-z0-9]+)([a-z0-9]{10})$#i',$goodPassword,$matches);
      $dbHashType = $matches[1];
      $goodHash = $matches[2];
      $salt = $matches[3];

      if($challenge !== null AND preg_match('#^{([a-z0-9]+)}([a-z0-9]+)$#i',$password,$matches) > 0 AND array_key_exists($challenge,$this->validChallenges)) {
        // Authentification avec challenge response
        // On récupère le type de hash
        unset($this->validChallenges[$challenge]);
        $hashType = $matches[1];
        $hash = $matches[2];
        if($hash == hash_hmac($hashType,$goodHash,$challenge)) {
          // L'utilisateur est authentifié
          $auth = true;
        }
      } else {
        // Authentification classique
        if(hash_hmac($dbHashType,$password,$salt) == $goodHash) {
          // L'utilisateur est authentifié
          $auth = true;
        }
      }
    }

    if($auth === true) {
      $this->setUserId($id);
      $this->user = new User($id);
      $core->logger->info($id,'Utilisateur authentifié');
      $core->logger->info($this->user,'Utilisateur authentifié');
    }
  }

  public function getUserSalt($email) {
    $core = BdfCore::getInstance();
    $salt = NULL;
    $res = $core->db->query("SELECT ".$core->getConfig('sgbd','prefix')."_User.password FROM ".$core->getConfig('sgbd','prefix')."_User WHERE ".$core->getConfig('sgbd','prefix')."_User.email = '".$core->utils->escapeStr($email)."' LIMIT 1");
    if($core->db->numRows($res) > 0) {
      $password = $core->db->result($res,0);
      if(preg_match('#^{([a-z0-9]+)}([a-z0-9]+)([a-z0-9]{10})$#i',$password,$matches) > 0)
        $salt = $matches[3];
    }

    return $salt;
  }

  private function getValidChallenges() {
    if(isset($this->values['bdfChallenge'])) {
      $this->validChallenges = $this->values['bdfChallenge'];
      unset($this->values['bdfChallenge']);
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
      $this->values['bdfChallenge'] = $this->validChallenges;

    $_SESSION = $this->values;

  }

}

?>
