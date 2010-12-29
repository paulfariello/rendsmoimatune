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
        $debitor  = array();
        $creditor = array();

        $users = $this->_event->getUsers();
        foreach($users as $user) {
            $balance = $this->_event->getBalance($user);
            if ($balance > 0) {
                $creditor[] = $user;
            } elseif($balance < 0) {
                $debitor[] = $user;
            }
        }

        /** uasort($debitor);
        uasort($creditor);

        foreach($debitor as $debitorId => $due) {
            foreach($creditor as $creditorId => $amount) {
                if ($amount >= $due) {
                    $debt = new Debt(User::getRepository()->find($debitorId), User::getRepository()->find($creditorId), $due);
                    $debts->add($debt);
                    $creditor[$creditorId] -= $due;
                    break;
                }
            }
        } **/

        return array();
    }

}
