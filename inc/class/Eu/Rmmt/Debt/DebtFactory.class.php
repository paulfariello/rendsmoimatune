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
use Eu\Rmmt\Event;
use Eu\Rmmt\User;


/**
 * DebtFactory
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\Debt
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class DebtFactory
{
    private $_event;

    public function __construct(Event $event) {
        $this->_event = $event;
    }

    public function getEvent()
    {
        return $this->_event;
    }

    public function setEvent(Event $event)
    {
        $this->_event = $event;
    }

    /**
     * Create debts
     *
     * @return void
     *
     */
    public function createDebts()
    {
        // Array[userId] = $amount;
        $payed    = array();
        $due      = array();
        $creditor = array();
        $debitor  = array();
        $debts    = new ArrayCollection();

        foreach($this->_event->getExpenditures() as $expenditure) {
            foreach($expenditure->getPayers() as $payer) {
                $id = $payer->getUser()->getId();
                if (!isset($payed[$id])) {
                    $payed[$id] = 0;
                }
                $payed[$id] += $payer->getAmount();
            }

            foreach($expenditure->getBeneficiaries() as $beneficiary) {
                $id = $beneficiary->getUser()->getId();
                if (!isset($due[$id])) {
                    $due[$id] = 0;
                }
                $due[$id] += $beneficiary->getAmount();
            }
        }

        if (array_sum($payed) != array_sum($due)) {
            throw new \Exception("Unexpected difference between total paid and total received");
        }

        $usersId = array_keys($payed) + array_keys($due);
        foreach($usersId as $id) {
            if (!isset($due[$id])) {
                $due[$id] = 0;
            }
            if (!isset($payed[$id])) {
                $payed[$id] = 0;
            }

            $balance = $due[$id] - $payed[$id];
            if ($balance > 0) {
                $debitor[$id] = $balance;
            } elseif($balance < 0) {
                $creditor[$id] = -$balance;
            }
        }

        asort($debitor);
        asort($creditor);

        foreach($debitor as $debitorId => $due) {
            foreach($creditor as $creditorId => $amount) {
                if ($amount >= $due) {
                    $debt = new Debt(User::getRepository()->find($debitorId), User::getRepository()->find($creditorId), $due);
                    $debts->add($debt);
                    $creditor[$creditorId] -= $due;
                    break;
                }
            }
        }

        return $debts;
    }

}
