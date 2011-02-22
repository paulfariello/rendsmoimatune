<?php
/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of Rendsmoimatune.
 *
 * Rendsmoimatune is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rendsmoimatune is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rendsmoimatune.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  Rendsmoimatune
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: $revision$
 * @link
 */

namespace Eu\Rmmt;

/**
 * Tag
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class Tag
{
    private $_id;
    private $_name;
    private $_expenditures;

    public function __construct($name)
    {
        $this->_name = $name;
    }

    public function getId()
    {
        return $this->_id;
    }

    public function getName()
    {
        return $this->_name;
    }

    public function setName($name)
    {
        $this->_name = $name;
    }

    public function getExpenditures()
    {
        return $this->_expenditures;
    }

    public function addExpenditures(Expenditure $expenditure)
    {
        $this->_expenditures->add($expenditure);
    }

    public function removeExpenditures(Expenditure $expenditure)
    {
        $this->_expenditures->removeElement($expenditure);
    }

}