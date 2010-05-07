<?php

namespace Bdf;

class SmartyAdapter implements \Bdf\ITemplatesEngine {

  private $skin = null;
  const EXTENSION = ".tpl";
  private $smartyInstance = null;

  public function setSkin($skin) {
      $skinDir = ROOT.\Bdf\Core::getInstance()->getConfig("site","templates_dir").$skin.'/';
      if(file_exists($skinDir) AND is_dir($skinDir)) {
        $this->skin = $skin;
      }
      if($this->smartyInstance !== null) {
        $this->smartyInstance->template_dir = ROOT.\Bdf\Core::getInstance()->getConfig("site","templates_dir").$this->skin.'/';
      }
  }

  public function display($fileName) {
    $this->smartyInstance->display($fileName.self::EXTENSION);
  }

  public function initialization() {
    require_once(COTS."smarty/".\Bdf\Core::getInstance()->getConfig('templates','version')."/Smarty.class.php");
    $this->smartyInstance = new \Smarty();
    $this->smartyInstance->template_dir = ROOT.\Bdf\Core::getInstance()->getConfig("site","templates_dir").$this->skin.'/';
    $this->smartyInstance->compile_dir  = ROOT."templates_c/";
    if(\Bdf\Core::getInstance()->getConfig('logger','level') == 'Bdf::DEBUG') {
      $this->smartyInstance->debugging = true;
    } else {
      $this->smartyInstance->debugging = false;
    }
    $this->registerUtilsFunctions();
  }

  private function registerUtilsFunctions() {
   $utils = new \ReflectionClass('Bdf\Utils');
   $methods = $utils->getMethods();
   foreach($methods as $method) {
     if(!$method->isConstructor() AND !$method->isDestructor() AND substr($method->name,0,2) != "__") {
       $this->smartyInstance->register_function($method->name,array($method->class, $method->name));
     }
   }
  }

  public function assign($name, $value) {
    $this->smartyInstance->assign($name,$value);
  }

  public function assignByRef($name, &$value) {
    $this->smartyInstance->assign_by_ref($name,$value);
  }

  public function getSmartyInstance() {
    return $this->smartyInstance();
  }
}

?>
