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
    private $_fromUser;
    private $_toUser;
    private $_event;

    public function  __construct(Event $event, User $fromUser, User $toUser, $amount)
    {
        $this->_event    = $event;
        $this->_fromUser = $fromUser;
        $this->_toUser   = $toUser;
        $this->_amount   = $amount;
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

    public function getFromUser()
    {
        return $this->_fromUser;
    }

    public function setFromUser(User $fromUser)
    {
        $this->_fromUser = $fromUser;
    }

    public function getToUser()
    {
        return $this->_toUser;
    }

    public function setToUser(User $toUser)
    {
        $this->_toUser = $toUser;
    }

    public function getEvent()
    {
        return $this->_event;
    }

    public function setEvent(Event $event)
    {
        $this->_event = $event;
    }



}