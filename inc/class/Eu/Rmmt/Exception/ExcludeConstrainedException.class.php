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
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.rendsmoimatune.eu
 */

namespace Eu\Rmmt\Exception;

/**
 * ExcludeConstrainedException
 *
 * @category Class
 * @author   Paul Fariello <paul.fariello@gmail.com>
 */
class ExcludeConstrainedException extends \Exception {
    private $_expenditures;
    private $_repayments;

    public function  __construct(Array $expenditures, Array $repayments)
    {
        $this->_expenditures = $expenditures;
        $this->_repayments = $repayments;
    }

    public function getExpenditures()
    {
        return $this->_expenditures;
    }

    public function getRepayments()
    {
        return $this->_repayments;
    }
}
