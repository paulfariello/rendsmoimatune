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
use Eu\Rmmt\Account;
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
    private $_account;

    public function __construct(Account $account) {
        $this->_account = $account;
    }

    public function getAccount()
    {
        return $this->_account;
    }

    public function setAccount(Account $account)
    {
        $this->_account = $account;
    }

    /**
     * Compare deux utilisateurs par rapport à leur balances respectives.
     */
    private function _sortUsers($a, $b)
    {
        if ($a['balance'] == $b['balance']) {
            return 0;
        }
        return ($a['balance'] > $b['balance']) ? -1 : 1;
    }

    private function _resortFirstUser(array $users)
    {
        $first = $users[0];
        array_shift($users);

        if (sizeof($users) < 1) {
            return array($first);
        }

        $i = 0;
        while($i < sizeof($users) AND $first['balance'] < $users[$i]['balance']) {
            $i++;
        }
        
        return array_merge(array_slice($users, 0, $i), array($first), array_slice($users, $i));
    }

    /**
     * Create debts
     *
     * @return void
     *
     */
    public function createDebts()
    {
        $debts = new ArrayCollection();
        $debitors  = array();
        $creditors = array();

        $users = $this->_account->getUsers();
        foreach($users as $user) {
            $balance = $this->_account->getBalance($user);
            if ($balance > 0) {
                $creditors[] = array('user'=>$user, 'balance'=>$balance);
            } elseif($balance < 0) {
                $debitors[] = array('user'=>$user, 'balance'=>-$balance);
            }
        }

        usort($debitors, array($this, "_sortUsers"));
        usort($creditors, array($this, "_sortUsers"));

        while (!empty($debitors) AND !empty($creditors)) {
            $amount = min($creditors[0]['balance'], $debitors[0]['balance']);
            $debt = new Debt($debitors[0]['user'], $creditors[0]['user'], $amount);
            $debts->add($debt);

            $creditors[0]['balance'] -= $amount;
            $debitors[0]['balance'] -= $amount;

            // On met a jour les tableaux
            if ($creditors[0]['balance'] == 0) {
                array_shift($creditors);                    
            } else {
                $creditors = $this->_resortFirstUser($creditors);
            }

            if ($debitors[0]['balance'] == 0) {
                array_shift($debitors);                    
            } else {
                $debitors = $this->_resortFirstUser($debitors);
            }
        }

        return $debts;
    }

}
