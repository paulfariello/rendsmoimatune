<?php

/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of BotteDeFoin.
 *
 * BotteDeFoin is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * BotteDeFoin is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with BotteDeFoin.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

namespace Bdf;

/**
 * Logger
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */
class Logger implements \Doctrine\DBAL\Logging\SQLLogger
{
    private static $_instance;
    const DEBUG = 4;
    const INFO = 3;
    const WARN = 2;
    const ERROR = 1;
    const FATAL = 0;

    private static $_levelStr = array("FATAL","ERROR","WARN","INFO","DEBUG");

    private $_store = array();
    private $_noDebug = false;
    private $_console = false;
    private $_phpTrigger = true;


    /**
     * Constructeur
     *
     * @return Logger
     */
    private function __construct()
    {

    }

    /**
     * Accesseur à l'instance de Logger
     *
     * @return Logger
     */
    public static function getInstance()
    {
        if (!isset(self::$_instance)) {
            $c = __CLASS__;
            self::$_instance = new $c;
        }

        return self::$_instance;
    }

    /**
     * Redefinition de __clone pour interfire le clonnage de l'instance de Logger
     *
     * @return void
     */
    public function __clone()
    {
        trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
    }

    /**
     * Log un événement
     *
     * @param mixed  $value La valeur du log
     * @param int    $level Le niveau du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    private function _log($value, $level, $title)
    {
        if ($level <= $this->level) {
            $this->_store($value, $level, $title);
            $this->_write($value, $level, $title);
            $this->_phpTrigger($value, $level, $title);
        }
    }

    /**
     * Ecriture d'un log dans un fichier
     *
     * @param mixed  $value La valeur du log
     * @param int    $level Le niveau du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    private function _write($value, $level, $title)
    {
        //TODO
    }

    /**
     * Enregistre un log
     *
     * @param mixed  $value La valeur du log
     * @param int    $level Le niveau du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    private function _store($value, $level, $title)
    {
        $this->_store[] = array("title"=>$title,"value"=>$value,"backtrace"=>debug_backtrace(),"level"=>$level);
    }

    /**
     * Transforme un log en erreur php
     *
     * @param mixed  $value La valeur du log
     * @param int    $level Le niveau du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    private function _phpTrigger($value, $level, $title)
    {
        if ($this->_phpTrigger === true) {
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
            trigger_error($value, $phpLevel);
        }
    }

    /**
     * Transforme un tableau de backtrace en html
     *
     * @param array $backtrace Un tableau correspondant au backtrace
     *
     * @return string
     */
    private function _backtrace($backtrace)
    {
        $i = 1;
        $return = "<dl>";
        foreach ($backtrace as $trace) {
            // On trace pas tout ce qui se passe dans cette class sauf si toutes les traces sont dedans, à ce moment on affiche la dernière
            if (isset($trace['class']) AND $trace['file'] == __FILE__) continue;

            if (!empty($trace['args']))
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

    /**
     * Génère un log de debug
     *
     * @param mixed  $value La valeur du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    public function debug($value, $title ="Debug")
    {
        $this->_log($value, self::DEBUG, $title);
    }

    /**
     * Génère un log info
     *
     * @param mixed  $value La valeur du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    public function info($value, $title ="Info")
    {
        $this->_log($value, self::INFO, $title);
    }

    /**
     * Génère un log warning
     *
     * @param mixed  $value La valeur du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    public function warn($value, $title ="Warn")
    {
        $this->_log($value, self::WARN, $title);
    }

    /**
     * Génère un log erreur
     *
     * @param mixed  $value La valeur du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    public function error($value, $title ="Error")
    {
        $this->_log($value, self::ERROR, $title);
    }

    /**
     * Génère un log fatal
     *
     * @param mixed  $value La valeur du log
     * @param string $title Le titre du log
     *
     * @return void
     */
    public function fatal($value, $title ="Fatal")
    {
        $this->_log($value, self::FATAL, $title);
    }

    /**
     * Log une requète SQL
     *
     * @param string $sql    La requète SQL
     * @param array  $params Les paramètres de la requète SQL
     *
     * @return void
     */
    public function logSql($sql, array $params = null)
    {
        $this->_log($sql, self::DEBUG, "Debug SQL");
    }

    /**
     * Change le niveau de log minimum à afficher
     *
     * @param int $level Le niveau de log
     *
     * @return void
     */
    public function setLevel($level)
    {
        if (is_string($level)) {
            $level = constant(sprintf('self::%s', $level));
        } else {
            $level =(int)$level;
        }

        if ($level < 5 AND $level >= 0) {
            $this->level = $level;
        }
    }

    /**
     * Choisir d'afficher la console ou non
     *
     * @param boolean $console On affiche la console
     *
     * @return void
     */
    public function console($console)
    {
        $this->_console = (boolean)$console;
    }



    /**
     * Créer une chaine de caractère représentant une variable
     *
     * @param mixed $value La variable
     * 
     * @return string
     */
    private function _adaptedPrint($value)
    {
        if (is_array($value)) {
            return print_r($value, true);
        } elseif (is_bool($value)) {
            if ($value === true)
                return "true";
            else
                return "false";
        } elseif (is_object($value)) {
            if (method_exists($value, "__toString")) {
                return $value->__toString();
            } else {
                return print_r($value, true);
            }
        } else {
            return $value;
        }
    }

    /**
     * Destructeur
     *
     * @return void
     */

    public function __destruct()
    {
        if (!$this->_noDebug && $this->_console) {
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
                                        <?php foreach($this->_store as $log): ?>
                            <li class="log <?=strtolower(self::$levelStr[$log['level']])?>">
                                <span class="log-title"><?=$log['title']?> :</span><span class="backtrace-link">backtrace</span>
                                <div class="log-content hidden">
                                    <pre><?=$this->_adaptedPrint($log['value'])?></pre>
                                </div>
                                <div class="backtrace hidden">
                                                    <?=$this->_backtrace($log['backtrace'])?>
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

    /**
     * Desactive le debug
     *
     * @return void
     */
    public function setNoDebug()
    {
        $this->_noDebug = true;
    }
}
