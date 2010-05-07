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

    public static function makeUrl($url,$type = NULL) {
      if(isset($url[0]) AND $url[0] == '/') {
        $url = substr($url, 1);
      }
      $core = \Bdf\Core::getInstance();
      switch($type) {
        case "js":
          return $core->getConfig('site','url').$core->getConfig('site','javascript_dir').$core->getConfig('site','skin').'/'.$url;
          break;
        case "css":
          return $core->getConfig('site','url').$core->getConfig('site','style_dir').$core->getConfig('site','skin').'/'.$url;
          break;
        case "img":
          return $core->getConfig('site','url').$core->getConfig('site','image_dir').$core->getConfig('site','skin').'/'.$url;
          break;
        default:
          return \Bdf\Core::getInstance()->getConfig('site','url').$url;

      }
      if($type != NULL) {
      } else {
      }
    }

    public static function hashPassword($password) {
      $algo = \Bdf\Core::getInstance()->getConfig("sgbd","hash");
      $salt = uniqid(mt_rand(), false);
      $hash = hash_hmac($algo,$password, $salt);
      return '{'.$algo.'}'.$hash.$salt;
    }

    public static function comparePassword($password, $hash){
      $hashLen = array('SHA256' => 64);
      $algo = substr($hash, 1, strpos($hash, '}')-1);

      if(!isset($hashLen[$algo])) {
        throw new \Exception("L'algorithme de hashage n'est pas supporté");
      }

      $oldHash = substr($hash, strpos($hash, '}')+1, $hashLen[$algo]);
      $salt = substr($hash, strpos($hash, '}')+$hashLen[$algo]+1,strlen($hash));
      return $oldHash == hash_hmac($algo, $password, $salt);
    }

    public static function isCurrentPage($url) {
      return strstr($url, $_SERVER['REQUEST_URI']) !== false;
    }
}

?>
