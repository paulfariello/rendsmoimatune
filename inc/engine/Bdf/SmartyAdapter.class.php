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
 * SmartyAdapter
 *
 * Classe d'adaptation de Smarty à BotteDeFoin
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */
class SmartyAdapter implements \Bdf\ITemplatesEngine
{
    const EXTENSION = ".tpl";


    private $_skin           = null;
    private $_smartyInstance = null;
    private $_functions      = array(
        'makeUrl',
        'isCurrentPage'
    );
    private $_modifiers      = array(
        'intToByteQuantity'
    );

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::setSkin()}
     *
     * @param string $skin @see \Bdf\ITemplatesEngine::setSkin()
     *
     * @return void
     */
    public function setSkin($skin)
    {
        $skinDir = ROOT.\Bdf\Core::getInstance()->getConfig("templates", "dir").$skin.'/';
        if (file_exists($skinDir) AND is_dir($skinDir)) {
            $this->_skin = $skin;
        }
        if ($this->_smartyInstance !== null) {
            $this->_smartyInstance->template_dir = ROOT.\Bdf\Core::getInstance()->getConfig("templates", "dir").$this->_skin.'/';
        }
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::display()}
     *
     * @param string $fileName @see \Bdf\ITemplatesEngine::display()
     *
     * @return void
     */
    public function display($fileName)
    {
        $this->_smartyInstance->display($fileName.self::EXTENSION);
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::initialization()}
     *
     * @return void
     */
    public function initialization()
    {
        include_once COTS."smarty/".\Bdf\Core::getInstance()->getConfig('templates', 'release')."/Smarty.class.php";
        $this->_smartyInstance = new \Smarty();
        $this->_smartyInstance->template_dir = ROOT.\Bdf\Core::getInstance()->getConfig("templates", "dir").$this->_skin.'/';
        $this->_smartyInstance->compile_dir  = ROOT.\Bdf\Core::getInstance()->getConfig("templates", "compiled_dir");
        if (\Bdf\Core::getInstance()->getConfig('logger', 'level') == 'DEBUG') {
            $this->_smartyInstance->debugging = true;
        } else {
            $this->_smartyInstance->debugging = false;
        }
        $this->_registerUtilsFunctions();
        $this->_registerVariableModifier();
    }

    /**
     * Encapsulation de {@link \Bdf\Utils::makeUrl()}
     *
     * @param array  $params @see \Bdf\Utils::makeUrl()
     * @param Smarty $smarty instance de Smarty
     *
     * @return @see \Bdf\Utils::makeUrl()
     */
    public function utilsMakeUrl($params, $smarty)
    {
        if (isset($params['type'])) {
            return \Bdf\Utils::makeUrl($params['url'], $params['type']);
        } else {
            return \Bdf\Utils::makeUrl($params['url']);
        }
    }

    /**
     * Encapsulation de {@link \Bdf\Utils::isCurrentPage()}
     *
     * @param array  $params @see \Bdf\Utils::isCurrentPage()
     * @param Smarty $smarty instance de Smarty
     *
     * @return @see \Bdf\Utils::isCurrentPage()
     */
    public function utilsIsCurrentPage($params, $smarty)
    {
        return \Bdf\Utils::isCurrentPage($params['page']);
    }

    /**
     * Encapsulation de {@link \Bdf\Utils::intToByteQuantity()}
     *
     * @param array  $params @see \Bdf\Utils::intToByteQuantity()
     * @param Smarty $smarty instance de Smarty
     *
     * @return @see \Bdf\Utils::intToByteQuantity()
     */
    public function utilsIntToByteQuantity($param)
    {
        return \Bdf\Utils::intToByteQuantity($param);
    }

    /**
     * Enregistrement des méthodes de {@link \Bdf\Utils} dans smarty
     *
     * @return void
     */
    private function _registerUtilsFunctions()
    {
        $utils = new \ReflectionClass('Bdf\Utils');
        $smartyAdapter = new \ReflectionClass(__class__);
        $methods = $utils->getMethods();
        foreach ($methods as $method) {
            if (!$method->isConstructor() AND !$method->isDestructor() AND in_array($method->name, $this->_functions) AND substr($method->name, 0, 2) != "__") {
                $methodName = "utils".ucfirst($method->name);
                if (method_exists($this, $methodName)) {
                    $this->_smartyInstance->register->templateFunction($method->name, array($this, $methodName));
                } else {
                    \Bdf\Logger::getInstance()->error("La méthode ".$method->name." n'est pas definie dans SmartyAdapter->".$methodName);
                }
            }
        }
    }

    /**
     * Enregistrement des méthodes de {@link \Bdf\Utils} dans smarty
     *
     * @return void
     */
    private function _registerVariableModifier()
    {
        $utils = new \ReflectionClass('Bdf\Utils');
        $smartyAdapter = new \ReflectionClass(__class__);
        $methods = $utils->getMethods();
        foreach ($methods as $method) {
            if (!$method->isConstructor() AND !$method->isDestructor() AND in_array($method->name, $this->_modifiers) AND substr($method->name, 0, 2) != "__") {
                $methodName = "utils".ucfirst($method->name);
                if (method_exists($this, $methodName)) {
                    $this->_smartyInstance->register->modifier($method->name, array($this, $methodName));
                } else {
                    \Bdf\Logger::getInstance()->error("La méthode ".$method->name." n'est pas definie dans SmartyAdapter->".$methodName);
                }
            }
        }
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::assign()}
     *
     * @param string $name  @see \Bdf\ITemplatesEngine::assign()
     * @param string $value @see \Bdf\ITemplatesEngine::assign()
     *
     * @return void
     */
    public function assign($name, $value)
    {
        $this->_smartyInstance->assign($name, $value);
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::assignByRef()}
     *
     * @param string $name   @see \Bdf\ITemplatesEngine::assignByRef()
     * @param string &$value @see \Bdf\ITemplatesEngine::assignByRef()
     *
     * @return void
     */
    public function assignByRef($name, &$value)
    {
        $this->_smartyInstance->assign_by_ref($name, $value);
    }

    /**
     * Accesseur à l'instance de smarty
     *
     * @return Smarty
     */
    public function getSmartyInstance()
    {
        return $this->smartyInstance();
    }
}
