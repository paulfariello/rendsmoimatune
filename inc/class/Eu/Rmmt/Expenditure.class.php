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
use DateTime;
use Doctrine\Common\Collections\ArrayCollection;

/**
 * Expenditure
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class Expenditure
{
    private $_id;
    private $_name;
    private $_date;
    private $_amount;
    private $_payers;
    private $_beneficiaries;
    private $_tags;
    private $_event;

    public function __construct(Event $event, $name, $amount)
    {
        $this->_event          = $event;
        $this->_name           = $name;
        $this->_amount         = $amount;
        $this->_payers         = new ArrayCollection();
        $this->_beneficiaries  = new ArrayCollection();
        $this->_tags           = new ArrayCollection();
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

    public function getPayers()
    {
        return $this->_payers;
    }

    public function addPayer(User $user, $amount)
    {
        $payer = new Payer($this, $user, $amount);
        $this->_payers->add($payer);
    }

    public function removePayer(User $user)
    {
        //TODO remove paying user
    }

    public function getBeneficiaries()
    {
        return $this->_beneficiaries;
    }

    public function addBeneficiary(User $user, $amount)
    {
        $beneficiary = new Beneficiary($this, $user, $amount);
        $this->_beneficiaries->add($beneficiary);
    }

    public function removeBeneficiary(User $user)
    {
        //TODO remove involved user
    }

    public function getTags()
    {
        return $this->_tags;
    }

    public function addTag(Tag $tag)
    {
        $this->_tags->add($tag);
    }

    public function removeTag(Tag $tag)
    {
        $this->_tags->removeElement($tag);
    }

    public function getEvent() {
        return $this->_event;
    }

    public function setEvent($event) {
        $this->_event = $event;
    }


}