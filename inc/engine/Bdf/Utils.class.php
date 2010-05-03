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

namespace Bdf;

class Utils {

    private static $instance = null;
    const RESSOURCE_IMAGE = 1;
    const RESSOURCE_TEMPLATE = 2;
    const RESSOURCE_CSS = 3;


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

    public static function getInstance() {
      if (self::$instance === null) {
        $c = __CLASS__;
        self::$instance = new $c;
      }

      return self::$instance;
    }

    public function __clone() {
      trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
    }


    public function makeUrl($url,$type = NULL) {
      if($type != NULL) {
        return \Bdf\Core::getInstance()->getConfig('site','url').$type.'/'.\Bdf\Session::getInstance()->getUser()->getSkin().'/'.$url;
      } else {
        return \Bdf\Core::getInstance()->getConfig('site','url').$url;
      }
    }
}

?>
