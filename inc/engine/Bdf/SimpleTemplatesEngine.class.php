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

class SimpleTemplatesEngine implements \Bdf\ITemplatesEngine {

  private $skin = null;
  private $vars = array();
  const TEMPLATES_DIR = "templates/";
  const EXTENSION = ".php";

  public function setSkin($skin) {
      if(file_exists(ROOT.self::TEMPLATES_DIR.$skin) && is_dir(ROOT.self::TEMPLATES_DIR.$skin)) {
        $this->skin = $skin;
      }
  }

  public function display($fileName) {
    foreach($this->vars as $name => $value) {
      ${$name} = $value;
    } 
    include(ROOT.self::TEMPLATES_DIR.$this->skin."/".$fileName.self::EXTENSION);
  }

  public function initialization() { }

  public function assignByRef($name,&$value) {
    $this->vars[$name] = &$value;
  }

  public function assign($name,$value) {
    $this->vars[$name] = $value;
  }

  public function insert($fileName,array $vars = array()) {
    foreach($this->vars as $name => $value) {
      ${$name} = $value;
    }
    foreach($vars as $name=>$value) {
      ${$name} = $value;
    }
    include(ROOT.self::TEMPLATES_DIR.$this->skin."/".$fileName);
  }
}

?>
