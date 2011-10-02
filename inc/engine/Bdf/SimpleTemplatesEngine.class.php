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
 * SimpleTemplatesEngine
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */
class SimpleTemplatesEngine implements \Bdf\ITemplatesEngine
{
    const TEMPLATES_DIR = "templates/";
    const EXTENSION = ".php";

    private $_skin = null;
    private $_vars = array();


    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::setSkin()}
     *
     * @param string $skin @see \Bdf\ITemplatesEngine::setSkin()
     *
     * @return void
     */
    public function setSkin($skin)
    {
        if (file_exists(ROOT.self::TEMPLATES_DIR.$skin) && is_dir(ROOT.self::TEMPLATES_DIR.$skin)) {
            $this->_skin = $skin;
        } else {
            // TODO throw exception
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
        foreach ($this->_vars as $name => $value) {
            ${$name} = $value;
        }
        include ROOT.self::TEMPLATES_DIR.$this->_skin."/".$fileName.self::EXTENSION;
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::fetch()}
     *
     * @param string $fileName @see \Bdf\ITemplatesEngine::fetch()
     *
     * @return void
     */
    public function fetch($fileName)
    {
        foreach ($this->_vars as $name => $value) {
            ${$name} = $value;
        }
        ob_start();
        include ROOT.self::TEMPLATES_DIR.$this->_skin."/".$fileName.self::EXTENSION;
        $return = ob_get_contents();
        ob_end_clean();
        return $return;
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::initialization()}
     *
     * @return void
     */
    public function initialization()
    {

    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::assignByRef()}
     *
     * @param string $name   @see \Bdf\ITemplatesEngine::assignByRef()
     * @param string &$value @see \Bdf\ITemplatesEngine::assignByRef()
     *
     * @return void
     */
    public function assignByRef($name,&$value)
    {
        $this->_vars[$name] = &$value;
    }

    /**
     * Inherited from {@link \Bdf\ITemplatesEngine::assign()}
     *
     * @param string $name  @see \Bdf\ITemplatesEngine::assign()
     * @param string $value @see \Bdf\ITemplatesEngine::assign()
     *
     * @return void
     */
    public function assign($name,$value)
    {
        $this->_vars[$name] = $value;
    }


    /**
     * Insert un autre templates
     *
     * @param string $fileName Le templates
     *
     * @return void
     */
    public function insert($fileName)
    {
        include ROOT.self::TEMPLATES_DIR.$this->_skin."/".$fileName;
    }
}
