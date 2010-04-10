<?php

/**
 * Nom de la class
 *
 *
 * <p>Description de la class</p>
 *
 * @name nom
 * @author Needle <paul.fariello@gmail.com>
 * @link
 * @copyright Paul Fariello 2007
 * @version 1.0.0
 * @package Nom du package
 */

abstract class BdfUser {

    /*~*~*~*~*~*~*~*~*~*~*/
    /*  1. propriétés    */
    /*~*~*~*~*~*~*~*~*~*~*/

    /**
     * @var type
     * @desc description
     */
    
    protected $id;
    protected $email = 'invité';
    protected $password;

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


    public function save() {
      $core = BdfCore::getInstance();
      if($this->id == 0) {
        $core->db->query("INSERT INTO ".$core->getConfig('sgbd','prefix')."_User (id,email,password) VALUES (NULL,'".$core->utils->escapeStr($this->email)."','".$core->utils->espaceStr($this->password)."');");
        $this->id = $core->db->insertId();
        $core->db->query("INSERT INTO user (id) VALUES (".$this->id.")");
      } else {
        $core->db->query("UPDATE ".$core->getConfig('sgbd','prefix')."_User SET email = '".$core->utils->escapeStr($this->email)."', password = '".$core->utils->espaceStr($this->password)."') WHERE ".$core->getConfig('sgbd','prefix')."_User.id = ".(int)$this->id." LIMIT 1;");
      }
    }


    public function getInfo() {
      $core = BdfCore::getInstance();
      $res = $core->db->query("
        SELECT
        ".$core->getConfig('sgbd','prefix')."_User.email
        FROM
          ".$core->getConfig('sgbd','prefix')."_User
        WHERE
          ".$core->getConfig('sgbd','prefix')."_User.id = '".(int)$this->id."'
        LIMIT 1");

      list(
        $this->email
      ) = $core->db->fetchRow($res);

    }

    public function getEmail() {
      return $this->email;
    }

    public function getId() {
      return $this->id;
    }

    public function setPassword($password) {
      $this->password = '{'.$core->getConfig('sgbd','hash').'}'.hash_hmac($core->getConfig('sgbd','hash'),$password,BdfSession::getRandomSalt());
    }
    /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
    /*  2.1 méthodes privées   */
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

    /**
     * Nom de la fonction
     *
     * <p>Description de la fonction</p>
     *
     * @name maClass::maFonction()
     * @param void
     * @return void
     */

    public function getSkin() {
      return $this->skin;
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


    }

}

?>
