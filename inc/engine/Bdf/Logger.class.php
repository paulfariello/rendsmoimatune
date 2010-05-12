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


class Logger implements \Doctrine\DBAL\Logging\SQLLogger {

  /*~*~*~*~*~*~*~*~*~*~*/
  /*  1. propriétés    */
  /*~*~*~*~*~*~*~*~*~*~*/

  /**
   * @var type
   * @desc description
   */

  private static $instance;
  const DEBUG = 4;
  const INFO = 3;
  const WARN = 2;
  const ERROR = 1;
  const FATAL = 0;

  private static $levelStr = array("FATAL","ERROR","WARN","INFO","DEBUG");
  
  private $store = array();
  private $noDebug = false;
  private $console = false;
  private $phpTrigger = true;

  /**
   * @var type
   * @desc description
   */

  public $variable_public;



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

  private function log($value,$level,$title) {
    if($level <= $this->level) {
      $this->store($value,$level,$title);
      $this->write($value,$level,$title);
      $this->phpTrigger($value,$level,$title);
    }
  }

  private function write($value,$level,$title) {
    // Enregistrement du log dans un fichier

  }

  private function phpTrigger($value, $level, $title) {
    if($this->phpTrigger === true) {
      $phpLevel = E_USER_NOTICE;
      switch($level) {
        case self::FATAL:
        case self::ERROR:
          $phpLevel = E_USER_ERROR;
          break;
        case self::WARN:
          $phpLevel = E_USER_WARNING;
          break;
        case self::INFO:
        case self::DEBUG:
          $phpLevel = E_USER_NOTICE;
          break;
      }
      trigger_error($value,$phpLevel);
    }
  }

  private function backtrace($backtrace) {
    $i = 1;
    $return = "<dl>";
    foreach($backtrace as $trace) {
      // On trace pas tout ce qui se passe dans cette class sauf si toutes les traces sont dedans, à ce moment on affiche la dernière
      if(isset($trace['class']) AND $trace['file'] == __FILE__) continue;
	  
      if(!empty($trace['args']))
        $return .= '<dt>';
      else
        $return .= '<dt>';

      $return .= '#'.$i.' '.$trace['function'];
      $return .= ' on '.$trace['file'];
      $return .= ' at '.$trace['line'];

      if(isset($trace['class']))
        $return .= ' in class '.$trace['class'];
      $i++;
    }
    return $return."</dl>";
  }
  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/
  /*  2.1 méthodes publiques */
  /*~*~*~*~*~*~*~*~*~*~*~*~*~*/

  public function debug($value,$title ="Debug") {
    $this->log($value,self::DEBUG,$title);
  }

  public function info($value,$title ="Info") {
    $this->log($value,self::INFO,$title);
  }
  public function warn($value,$title ="Warn") {
    $this->log($value,self::WARN,$title);
  }

  public function error($value,$title ="Error") {
    $this->log($value,self::ERROR,$title);
  }

  public function fatal($value,$title ="Fatal") {
    $this->log($value,self::FATAL,$title);
  }

  public function logSql($sql, array $params = null) {
    $this->log($sql,self::DEBUG,"Debug SQL");
  }

  public function setLevel($level) {
    if(is_string($level)) {
      $level = constant('self::'.$level);
    } else {
      $level =(int)$level;
    }

    if($level < 5 AND $level >= 0) {
      $this->level = $level;
    }
  }

  public function console($console) {
    $this->console = (boolean)$console;
  }
  
  private function store($value,$level,$title) {
    $this->store[] = array("title"=>$title,"value"=>$value,"backtrace"=>debug_backtrace(),"level"=>$level);
  }


  private function adaptedPrint($value) {
    if(is_array($value)) {
      return print_r($value,true);
    } elseif(is_bool($value)) {
      if($value === true)
        return "true";
      else
        return "false";
    } elseif(is_object($value)) {
      if(method_exists($value, "__toString")) {
        return $value->__toString();
      } else {
        return print_r($value,true);
      }
    } else {
      return $value;
    }
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
    if(!$this->noDebug && $this->console) {
?>
  			<div id="console-log">
          <div id="console-menu">
            <ul>
              <li id="console-menu-log">Log</li>
              <li id="console-menu-ajax">Ajax</li>
              <li id="console-menu-request">Request</li>
              <li id="console-menu-user">User</li>
            </ul>
          </div>
          <div id="console-content">
            <div id="console-tab-log" class="console-tab">
              <div id="console-log-menu">
                <ul>
                  <li id="console-log-menu-all" class="selected log-filter">All</li>
                  <li id="console-log-menu-debug" class="log-filter">Debug</li>
                  <li id="console-log-menu-info" class="log-filter">Info</li>
                  <li id="console-log-menu-warn" class="log-filter">Warn</li>
                  <li id="console-log-menu-error" class="log-filter">Error</li>
                  <li id="console-log-menu-fatal" class="log-filter">Fatal</li>
                </ul>
              </div>
              <ul id="console-log-content">
              <?php foreach($this->store as $log): ?>
                <li class="log <?=strtolower(self::$levelStr[$log['level']])?>">
                  <span class="log-title"><?=$log['title']?> :</span><span class="backtrace-link">backtrace</span>
                  <div class="log-content hidden">
                    <pre><?=$this->adaptedPrint($log['value'])?></pre>
                  </div>
                  <div class="backtrace hidden">
                    <?=$this->backtrace($log['backtrace'])?>
                  </div>      
                </li>  
              <?php endforeach; ?>
              </ul>
            </div>
            <div id="console-tab-ajax" class="console-tab">
            </div>
            <div id="console-tab-request" class="console-tab">
              <div id="post">
              </div>
              <div id="get">
              </div>
              <div id="file">
              </div>
            </div>
            <div id="console-tab-user" class="console-tab">
              <ul>
                <li><?=BdfCore::getInstance()->session->getUser()->getEmail();?></li>
                <li><?=BdfCore::getInstance()->session->getUser()->getId();?></li>
              </ul>
            </div>
          </div>
  			</div>
<?php
		}
  }

  public function setNoDebug() {
    $this->noDebug = true;
  }
}

?>
