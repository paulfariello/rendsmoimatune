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

class BdfUtils {

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


    }

    /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
    /*  2.1 méthodes privées   */
    /*~*~*~*~*~*~*~*~*~*~*~*~*~*/

    /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
    /*  2.1 méthodes publiques */
    /*~*~*~*~*~*~*~*~*~*~*~*~*~*/

    /**
     * escapeStr
     *
     * <p>Doit être utilisé à la place de toute autre fonction d'ajout de slash</p>
     *
     * @name BdfUtils::escapeStr()
     * @param void
     * @return void
     */

    public static function escapeStr($str) {
        return BdfCore::getInstance()->db->realEscapeString($str);
    }
	
    public static function makeUrl($url,$type = NULL) {
      if($type != NULL) {
        return BdfCore::getInstance()->getConfig('site','url').$type.'/'.BdfCore::getInstance()->session->getUser()->getSkin().'/'.$url;
      } else {
        return BdfCore::getInstance()->getConfig('site','url').$url;
      }
    }
}

?>
