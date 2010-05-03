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
  const EXTENSION = ".tpl";
  private $smartyInstance = null;

  public function setSkin($skin) {
      if(file_exists(self::DIR.$skin) && is_dir(self::DIR.$skin)) {
        $this->skin = $skin;
      }
  }

  public function display($fileName) {
    $this->smartyInstance->display($fileName.EXTENSION);
  }

  public function initialization() {
    require_once(COTS."smarty/".$this->getConfig('smarty','version')."/Smarty.class.php");
    $this->smartyInstance = new \Smarty();
    $this->smartyInstance->template_dir = ROOT."templates/".$this->session->getUser()->getSkin().'/';
    $this->smartyInstance->compile_dir  = ROOT."templates_c/";
    if($this->getConfig('logger','level') == 'Bdf::DEBUG') {
      $this->smartyInstance->debugging = true;
    } else {
      $this->smartyInstance->debugging = false;
    }

    $this->smartyInstance->assign("bdfUtils",$this->utils);
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
