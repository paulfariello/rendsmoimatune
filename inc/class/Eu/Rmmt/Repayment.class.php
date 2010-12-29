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
use \DateTime;
use \Bdf\Utils;

/**
 * Repayment
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class Repayment
{
    private $_id;
    private $_date;
    private $_amount;
    private $_payer;
    private $_beneficiary;
    private $_event;

    public function  __construct(Event $event, User $payer, User $beneficiary, $amount)
    {
        $this->_event       = $event;
        $this->_payer       = $payer;
        $this->_beneficiary = $beneficiary;
        $this->_amount      = $amount;
    }

    public function getId()
    {
        return $this->_id;
    }

    public function setId($id)
    {
        $this->_id = $id;
    }

    public function getDate()
    {
        return $this->_date;
    }

    public function setDate(DateTime $date)
    {
        $this->_date = $date;
    }

    public function getAmount()
    {
        return $this->_amount;
    }

    public function setAmount($amount)
    {
        $this->_amount = $amount;
    }

    public function getPayer()
    {
        return $this->_payer;
    }

    public function setPayer(User $fromUser)
    {
        $this->_payer = $fromUser;
    }

    public function getBeneficiary()
    {
        return $this->_beneficiary;
    }

    public function setBeneficiary(User $toUser)
    {
        $this->_beneficiary = $toUser;
    }

    public function getEvent()
    {
        return $this->_event;
    }

    public function setEvent(Event $event)
    {
        $this->_event = $event;
    }

    public function getDescription()
    {
        return Utils::getText('%1$s repaid %2$.2f€ to %3$s', $this->_payer->getName(), $this->_amount, $this->_beneficiary->getName());
    }

}
