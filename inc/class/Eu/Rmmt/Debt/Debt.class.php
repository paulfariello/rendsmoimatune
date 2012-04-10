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
 * @version  SVN: 145
 * @link     http://www.Rendsmoimatune.fr
 */

namespace Eu\Rmmt\Debt;
use Doctrine\Common\Collections\ArrayCollection;
use Bdf\Core;
use Eu\Rmmt\User;


/**
 * Debt
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\Debt
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class Debt
{
    private $_from;
    private $_to;
    private $_amount;

    public function __construct(User $from, User $to, $amount) {
        $amount = (int)$amount;

        if ($amount > 0) {
            $this->_from   = $from;
            $this->_to     = $to;
            $this->_amount = $amount;
        } elseif ($amount < 0) {
            $this->_from   = $to;
            $this->_to     = $from;
            $this->_amount = -$amount;
        } else {
            throw new \Exception("Cannot create a zeroed debt");
        }
    }

    public function getFrom()
    {
        return $this->_from;
    }

    public function setFrom(User $from)
    {
        $this->_from = $from;
    }

    public function getTo()
    {
        return $this->_to;
    }

    public function setTo(User $to)
    {
        $this->_to = $to;
    }

    public function getAmount()
    {
        return $this->_amount;
    }

    public function setAmount($amount)
    {
        $amount = (int)$amount;
        if ($amount > 0) {
            $this->_amount = $amount;
        } elseif ($amount < 0) {
            $to = $this->_from;
            $this->_from = $this->_to;
            $this->_to = $to;
            $this->amount = -$amount;
        } else {
            throw new \Exception("Cannot create a zeroed debt");
        }
    }
}
