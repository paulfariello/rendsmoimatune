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

namespace Eu\Rmmt;
use DateTime;
use Bdf\Utils;
use Doctrine\Common\Collections\ArrayCollection;
use Eu\Rmmt\Exception\RightException;
use Eu\Rmmt\Exception\ExcludeConstrainedException;

/**
 * Account
 *
 * @category Class
 * @author   Paul Fariello <paul.fariello@gmail.com>
 */
class Account
{
    private $_id;
    private $_name;
    private $_startDate;
    private $_endDate;
    private $_expenditures;
    private $_users;
    private $_repayments;
    private $_creator;
    private $_creationDate;

    public function  __construct($name, User $creator)
    {
        $this->_name            = $name;
        $this->_creator         = $creator;
        $this->_creationDate    = new DateTime();
        $this->_users           = new ArrayCollection();
        $this->_users->add($creator);
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

    public function getStartDate()
    {
        return $this->_startDate;
    }

    public function setStartDate(DateTime $startDate)
    {
        $this->_startDate = $startDate;
    }

    public function getEndDate()
    {
        return $this->_endDate;
    }

    public function setEndDate(DateTime $endDate)
    {
        $this->_endDate = $endDate;
    }

    public function getExpenditures($limit = null, $page = 1)
    {
        if (null != $limit) {
            return $this->_expenditures->slice(($page - 1) * $limit, $limit);
        } else {
            return $this->_expenditures;
        }
    }

    public function addExpenditure(Expenditure $expenditure)
    {
        $this->_expenditures->add($expenditure);
    }

    public function removeExpenditure(Expenditure $expenditure)
    {
        $this->_expenditures->removeElement($expenditure);
    }

    public function getUsers()
    {
        return $this->_users;
    }

    public function addUser(User $user)
    {
        if ( ! $this->_users->contains($user)) {
            $this->_users->add($user);
        }
    }

    public function removeUser(User $user)
    {
        $this->_users->removeElement($user);
    }

    public function getRepayments($limit = null)
    {
        if (null != $limit) {
            return $this->_repayments->slice(0, $limit);
        } else {
            return $this->_repayments;
        }
    }

    public function addRepayments(Repayment $repayment)
    {
        $this->_repayments->add($repayment);
    }

    public function removeRepayments(Repayment $repayment)
    {
        $this->_repayments->removeElement($repayment);
    }

    public function getCreator()
    {
        return $this->_creator;
    }

    public function getCreationDate()
    {
        return $this->_creationDate;
    }

    public function isCreator(User $user)
    {
        return $this->_creator->equals($user);
    }

    public function grantAccess(User $user)
    {
        if ($this->_users->contains($user)) {
            return true;
        }

        // If user has payed for an expenditure
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT count(p._id) FROM \Eu\Rmmt\Payer p INNER JOIN p._expenditure ex INNER JOIN p._user u INNER JOIN ex._account a WHERE u._id = ?1 AND a._id = ?2");
        $query->setParameter(1, $user->getId());
        $query->setParameter(2, $this->getId());
        $count = $query->getSingleScalarResult();
        if ($count > 0) {
            return true;
        }

        // If user is concerned by an expenditure
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT count(b._id) FROM \Eu\Rmmt\Beneficiary b INNER JOIN b._expenditure ex INNER JOIN b._user u INNER JOIN ex._account a WHERE u._id = ?1 AND a._id = ?2");
        $query->setParameter(1, $user->getId());
        $query->setParameter(2, $this->getId());
        $count = $query->getSingleScalarResult();
        if ($count > 0) {
            return true;
        }

    }

    public function getPayedAmount(User $user)
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(p._amount) FROM \Eu\Rmmt\Payer p INNER JOIN p._expenditure ex INNER JOIN ex._account a INNER JOIN p._user u WHERE u._id = :user AND a._id = :account");
        $query->setParameter("user", $user->getId());
        $query->setParameter("account", $this->getId());
        $amount = $query->getSingleScalarResult();
        if(null == $amount) {
            return 0;
        } else {
            return $amount;
        }
    }

    public function getRepaymentPayedAmount(User $user)
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(r._amount) FROM \Eu\Rmmt\Repayment r INNER JOIN r._account a INNER JOIN r._payer u WHERE u._id = :user AND a._id = :account");
        $query->setParameter("user", $user->getId());
        $query->setParameter("account", $this->getId());
        $amount = $query->getSingleScalarResult();
        if(null == $amount) {
            return 0;
        } else {
            return $amount;
        }
    }


    public function getOwesAmount(User $user)
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(b._amount) FROM \Eu\Rmmt\Beneficiary b INNER JOIN b._expenditure ex INNER JOIN ex._account a INNER JOIN b._user u WHERE u._id = :user AND a._id = :account");
        $query->setParameter("user", $user->getId());
        $query->setParameter("account", $this->getId());
        $amount = $query->getSingleScalarResult();
        if(null == $amount) {
            return 0;
        } else {
            return $amount;
        }
    }

    public function getRepaymentReceivedAmount(User $user)
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(r._amount) FROM \Eu\Rmmt\Repayment r INNER JOIN r._account a INNER JOIN r._beneficiary u WHERE u._id = :user AND a._id = :account");
        $query->setParameter("user", $user->getId());
        $query->setParameter("account", $this->getId());
        $amount = $query->getSingleScalarResult();
        if(null == $amount) {
            return 0;
        } else {
            return $amount;
        }
    }

    public function getMaxPayedAmount()
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(p._amount) as payed FROM \Eu\Rmmt\Payer p INNER JOIN p._expenditure ex INNER JOIN ex._account a INNER JOIN p._user u WHERE a._id = :account GROUP BY u._id ORDER BY payed DESC")->setMaxResults(1);
        $query->setParameter("account", $this->getId());
        $maxPayedAmout = $query->getSingleScalarResult();
        if(null == $maxPayedAmout) {
            return 0;
        } else {
            return $maxPayedAmout;
        }

    }

    public function getMaxOwesAmount()
    {
        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT sum(b._amount) as owes FROM \Eu\Rmmt\Beneficiary b INNER JOIN b._expenditure ex INNER JOIN ex._account a INNER JOIN b._user u WHERE a._id = :account GROUP BY u._id ORDER BY owes DESC")->setMaxResults(1);
        $query->setParameter("account", $this->getId());
        $maxOwesAmout = $query->getSingleScalarResult();
        if(null == $maxOwesAmout) {
            return 0;
        } else {
            return $maxOwesAmout;
        }
    }

    public function getBalance(User $user)
    {
        return $this->getPayedAmount($user) + $this->getRepaymentPayedAmount($user) - $this->getOwesAmount($user) - $this->getRepaymentReceivedAmount($user);
    }

    public function getTotalExpenditure() {
        $total = 0;
        foreach($this->getExpenditures() as $expenditure) {
            $total += $expenditure->getAmount();
        }
        return $total;
    }

    public function getTotalRepayment() {
        $total = 0;
        foreach($this->getRepayments() as $repayment) {
            $total += $repayment->getAmount();
        }
        return $total;
    }

    public static function getRepository()
    {
        return \Bdf\Core::getInstance()->getEntityManager()->getRepository(__CLASS__);
    }

    /**
     * Url management
     */

    public function getUrlDetail()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/');
    }

    public function getUrlDelete()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/delete.html');
    }

    public function getUrlExpendituresList($page = 1)
    {
        if ($page > 1) {
            return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/expenditures-list-page-'.(int)$page.'.html');
        } else {
            return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/expenditures-list.html');
        }
    }

    public function getUrlRepaymentsList($page = 1)
    {
        if ($page > 1) {
            return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/repayments-list-page-'.(int)$page.'.html');
        } else {
            return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/repayments-list.html');
        }
    }

    public function getUrlNewExpenditure()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/create-new-expenditure.html');
    }

    public function getUrlNewRepayment()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/create-new-repayment.html');
    }

    public function getUrlViewExpenditure(Expenditure $expenditure)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/expenditure/view-'.Utils::urlize($expenditure->getTitle()).'-'.$expenditure->getId().'.html');
    }

    public function getUrlEditExpenditure(Expenditure $expenditure)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/expenditure/edit-'.Utils::urlize($expenditure->getTitle()).'-'.$expenditure->getId().'.html');
    }

    public function getUrlDeleteExpenditure(Expenditure $expenditure)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/expenditure/delete-'.Utils::urlize($expenditure->getTitle()).'-'.$expenditure->getId().'.html');
    }

    public function getUrlViewRepayment(Repayment $repayment)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/repayment/view-'.Utils::urlize($repayment->getDescription()).'-'.$repayment->getId().'.html');
    }

    public function getUrlEditRepayment(Repayment $repayment)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/repayment/edit-'.Utils::urlize($repayment->getDescription()).'-'.$repayment->getId().'.html');
    }

    public function getUrlDeleteRepayment(Repayment $repayment)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/repayment/delete-'.Utils::urlize($repayment->getDescription()).'-'.$repayment->getId().'.html');
    }

    public function getUrlCashUp()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/cash-up.html');
    }

    public function getUrlRename()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/rename.html');
    }

    public function getUrlParticipants()
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/participants.html');
    }

    public function getUrlExclusion(User $user)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/exclude-'.Utils::urlize($user->getName()).'-'.$user->getId().'.html');
    }

    public function getUrlAutocompleteUser()
    {
        return Utils::makeUrl('ajax/autocomplete-user.php?aid='.$this->_id);
    }

    public function getUrlMergeRequest(User $firstUser, User $secondUser)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/merge-user-'.$firstUser->getId().'-with-'.$secondUser->getId().'.html');
    }

    public function getUrlMergeRequestFromIds($firstUser, $secondUser)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/merge-user-'.$firstUser.'-with-'.$secondUser.'.html');
    }

    public function getUrlAcceptMergeRequest(MergeRequest $mergeRequest, $token)
    {
        return Utils::makeUrl('my-accounts/'.Utils::urlize($this->_name).'-'.$this->_id.'/accept-merge-'.$mergeRequest->getId().'-'.$token.'.html');
    }

    public function getUrlLoadAllParticipants()
    {
        return Utils::makeUrl('ajax/account-participants.php?aid='.$this->_id);
    }

    /**
     * Access control
     */

    public function checkViewRight(User $user)
    {
        if (!$this->grantAccess($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't access this account."));
        }
    }

    public function checkCreateRight(User $user)
    {
        if (!$this->grantAccess($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't create expenditure neither repayment in this account."));
        }
    }

    public function checkDeleteRight(User $user)
    {
        if (!$this->_creator->equals($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't delete this account because you haven't created it."));
        }
    }

    public function checkRenameRight(User $user)
    {
        if (!$this->_creator->equals($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't rename this account because you haven't created it."));
        }
    }

    public function checkExcludeRight(User $currentUser, User $user)
    {
        if (!$this->grantAccess($currentUser)) {
            throw new RightException(\Bdf\Utils::getText("You can't exclude an user from this account."));
        }

        if ($this->_creator->equals($user)) {
            throw new RightException(\Bdf\Utils::getText("You can't exclude an user from his/her own account."));
        }

        $em = \Bdf\Core::getInstance()->getEntityManager();
        $query = $em->createQuery("SELECT e FROM \Eu\Rmmt\Expenditure e INNER JOIN e._payers p INNER JOIN e._beneficiaries b INNER JOIN p._user pu INNER JOIN b._user bu INNER JOIN e._account a WHERE a._id = :aid AND (pu._id = :uid OR bu._id = :uid)");
        $query->setParameter("uid", $user->getId());
        $query->setParameter("aid", $this->getId());
        $expenditures = $query->execute();

        $query = $em->createQuery("SELECT r FROM \Eu\Rmmt\Repayment r INNER JOIN r._payer pu INNER JOIN r._beneficiary bu INNER JOIN r._account a WHERE a._id = :aid AND (pu._id = :uid OR bu._id = :uid)");
        $query->setParameter("uid", $user->getId());
        $query->setParameter("aid", $this->getId());
        $repayments = $query->execute();

        if (count($repayments) + count($expenditures) > 0) {
            throw new ExcludeConstrainedException($expenditures, $repayments);
        }
    }
}
