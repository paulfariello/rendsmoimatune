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

class User extends BdfUser {

  /*~*~*~*~*~*~*~*~*~*~*/
  /*  1. propriétés    */
  /*~*~*~*~*~*~*~*~*~*~*/

  /**
   * @var type
   * @desc description
   */

  private $skin = 'default';

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

  public function __construct($id) {

    $core = BdfCore::getInstance();

    $this->id = $id;

    if($id > 0) {
      parent::getInfo();

      $res = $core->db->query("
        SELECT
          user.skin
        FROM
          user
        WHERE
          user.id = '".(int)$id."'
        LIMIT 1");

      list(
        $this->skin  
      ) = $core->db->fetchRow($res);
    }

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

  public function save() {
    // On enregistre d'abord avec la method de BotteDeFoin
    parent::save();

    // Ensuite on enregistre nos infos
  }

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
