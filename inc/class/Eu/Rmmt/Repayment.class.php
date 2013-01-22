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
use \Bdf\Utils;
use Eu\Rmmt\Exception\RightException;
use \DateTime;

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
    private $_account;
    private $_creator;

    public function  __construct(Account $account, User $payer, User $beneficiary, $amount, User $creator)
    {
        $this->_account       = $account;
        $this->_payer       = $payer;
        $this->_beneficiary = $beneficiary;
        $this->_amount      = $amount;
        $this->_creator        = $creator;
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

    public function getAccount()
    {
        return $this->_account;
    }

    public function setAccount(Account $account)
    {
        $this->_account = $account;
    }

    public function getDescription()
    {
        return Utils::getText('%1$s repaid %2$s to %3$s', $this->_payer->getName(), \Bdf\Utils::moneyFormat($this->_amount), $this->_beneficiary->getName());
    }

    /**
     * Url management
     */

    public function getUrlView()
    {
        return $this->_account->getUrlViewRepayment($this);
    }

    public function getUrlEdit()
    {
        return $this->_account->getUrlEditRepayment($this);
    }

    public function getUrlDelete()
    {
        return $this->_account->getUrlDeleteRepayment($this);
    }

    /**
     * Access control
     */

    public function checkViewRight(User $user)
    {
        try {
            $this->_account->checkViewRight($user);
        } catch(RightException $e) {
            throw new RightException(\Bdf\Utils::getText("You can't view this repayment"));
        }
    }

    public function checkEditRight(User $user)
    {
        if (!$this->_creator->equals($user) and !$this->_account->getCreator()->equals($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't edit this repayment because you are not creator of this repayment neither of this account"));
        }
    }

    public function checkDeleteRight(User $user)
    {
        if (!$this->_creator->equals($user) and !$this->_account->getCreator()->equals($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't delete this repayment because you are not creator of this repayment neither of this account"));
        }
    }
    public static function getRepository()
    {
        return \Bdf\Core::getInstance()->getEntityManager()->getRepository(__CLASS__);
    }

}
