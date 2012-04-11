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
use Bdf\Core;
use Bdf\Utils;
use Eu\Rmmt\Mail\MergeRequestMail;
use Eu\Rmmt\Exception\MergeAuthorizationException;
use Eu\Rmmt\Exception\InvalidMergeRequestTokenException;
use Eu\Rmmt\Exception\UnknownUserException;

/**
 * MergeRequest
 *
 * Class representing fusion between two users
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class MergeRequest extends Entity
{
    private $_id;
    private $_account                   = null;
    private $_firstUser                 = null;
    private $_secondUser                = null;
    private $_firstUserAgreement        = false;
    private $_secondUserAgreement       = false;
    private $_firstUserRequestToken     = null;
    private $_secondUserRequestToken    = null;
    private $_requester                 = null;
    private $_keepName                  = null;
    private $_keepEmail                 = null;

    public function  __construct(Account $account, User $firstUser, User $secondUser, User $requester)
    {
        $this->_account     = $account;
        $this->_firstUser   = $firstUser;
        $this->_secondUser  = $secondUser;
        $this->_requester   = $requester;
    }
    
    public function getId()
    {
        return $this->_id;
    }
    
    public function getAccount()
    {
        return $this->_account;
    }

    public function setAccount(Account $account)
    {
        $this->_account = $account;
    }

    public function getFirstUser()
    {
        return $this->_firstUser;
    }

    public function setFirstUser(User $user)
    {
        $this->_firstUser = $user;
    }

    public function getSecondUser()
    {
        return $this->_secondUser;
    }

    public function setSecondUser(User $user)
    {
        $this->_secondUser = $user;
    }

    private function _hasFirstUserAgreement()
    {
        return $this->_requester->equals($this->_firstUser) or $this->_requester->equals($this->_firstUser->getCreator()) or $this->_firstUserAgreement;
    }

    private function _hasSecondUserAgreement()
    {
        return $this->_requester->equals($this->_secondUser) or $this->_requester->equals($this->_secondUser->getCreator()) or $this->_secondUserAgreement;
    }

    public function checkMergeRight()
    {
        $agreement = true;

        $exception = new MergeAuthorizationException($this);
        if (!$this->_hasFirstUserAgreement()) {
            $exception->addRequiredAgreement($this->_firstUser);    
            $agreement = false;
        }
        if (!$this->_hasSecondUserAgreement()) {
            $exception->addRequiredAgreement($this->_secondUser);    
            $agreement = false;
        }
        
        if (!$agreement) {
            throw $exception;
        }
    }

    public function requestAgreements()
    {
        if (!$this->_hasFirstUserAgreement()) {
           $this->_firstUserRequestToken = $this->_generateRequestToken($this->_firstUser); 
           $this->_sendRequest($this->_firstUser, $this->_firstUserRequestToken, $this->_secondUser);
        }

        if (!$this->_hasSecondUserAgreement()) {
           $this->_secondUserRequestToken = $this->_generateRequestToken($this->_secondUser); 
           $this->_sendRequest($this->_secondUser, $this->_secondUserRequestToken, $this->_firstUser);
        } 
    }

    public function acceptMerge(User $user, $token)
    {
        if (($this->_firstUser->IsRegistered() AND $this->_firstUser->equals($user)) OR $user->equals($this->_firstUser->getCreator())) {
            if ($token == $this->_firstUserRequestToken) {
                $this->_firstUserAgreement = true; 
            } else {
                throw new InvalidMergeRequestTokenException($token);
            }
        } elseif (($this->_secondUser->IsRegistered() AND $this->_secondUser->equals($user)) OR $user->equals($this->_secondUser->getCreator())) {
            if ($token == $this->_secondUserRequestToken) {
                $this->_secondUserAgreement = true; 
            } else {
                throw new InvalidMergeRequestTokenException($token);
            }
        } else {
            throw new UnknownUserException($user->getId());
        }
    }

    public function keepName($user)
    {
        if ($user > 0 and $user < 3)
            $this->_keepName = (int)$user;
    }

    public function keepEmail($user)
    {
        if ($user > 0 and $user < 3)
            $this->_keepEmail = (int)$user;
    }

    public function doMerge()
    {
        $em = Core::getInstance()->getEntityManager();
        $keptUser = null;
        $deletedUser = null;

        $em->getConnection()->beginTransaction();

        try {

            // Choosing wich user to keep
            if ($this->_keepEmail == null) {
                if ($this->getFirstUser()->isRegistered() AND $this->getSecondUser()->isRegistered()) {
                    throw new \Eu\Rmmt\Exception\MergeException(Utils::getText("Cannot choose which account to keep"));
                } elseif ($this->getSecondUser()->isRegistered()) {
                    $keptUser = $this->getSecondUser();
                    $deletedUser = $this->getFirstUser();
                } else { // Weither first user is registered or none of them is.
                    $keptUser = $this->getFirstUser();
                    $deletedUser = $this->getSecondUser();
                }
            }  else {
                if ($this->_keepEmail == 1) {
                    $keptUser = $this->getFirstUser();
                    $deletedUser = $this->getSecondUser();
                } elseif($this->_keepEmail == 2) {
                    $keptUser = $this->getSecondUser();
                    $deletedUser = $this->getFirstUser();
                } else {
                    throw new \Eu\Rmmt\Exception\MergeException(Utils::getText("Cannot choose which account to keep"));
                }
            }

            // Setting name
            if ($this->_keepName == 1) {
                $keptUser->setName($this->getFirstUser()->getName());
            } elseif ($this->_keepName == 2) {
                $keptUser->setName($this->getSecondUser()->getName());
            }

            // Reowning expenditure and repayment
            $query = $em->createQuery('UPDATE Eu\Rmmt\Repayment r SET r._creator = :kept WHERE r._creator = :deleted');
            $query->setParameter('kept', $keptUser);
            $query->setParameter('deleted', $deletedUser);
            $query->execute();

            $query = $em->createQuery('UPDATE Eu\Rmmt\Expenditure e SET e._creator = :kept WHERE e._creator = :deleted');
            $query->setParameter('kept', $keptUser);
            $query->setParameter('deleted', $deletedUser);
            $query->execute();

            // Reowning repayment
            // Clean useless repayment
            $query = $em->createQuery('DELETE Eu\Rmmt\Repayment r WHERE (r._payer = :kept AND r._beneficiary = :deleted) OR (r._payer = :deleted1 AND r._beneficiary = :kept1)');
            $query->setParameter('kept', $keptUser);
            $query->setParameter('deleted', $deletedUser);
            $query->setParameter('kept1', $keptUser);
            $query->setParameter('deleted1', $deletedUser);
            $query->execute();

            $query = $em->createQuery('UPDATE Eu\Rmmt\Repayment r SET r._payer = :kept WHERE r._payer = :deleted');
            $query->setParameter('kept', $keptUser);
            $query->setParameter('deleted', $deletedUser);
            $query->execute();

            $query = $em->createQuery('UPDATE Eu\Rmmt\Repayment r SET r._beneficiary = :kept WHERE r._beneficiary = :deleted');
            $query->setParameter('kept', $keptUser);
            $query->setParameter('deleted', $deletedUser);
            $query->execute();

            // Merging collections 
            foreach($deletedUser->getPayers() as $payer) {
                $alreadyPayer = false;
                foreach ($payer->getExpenditure()->getPayers() as $keptPayer) {
                    if ($keptPayer->getUser()->equals($keptUser)) {
                        $alreadyPayer = true;
                        break; // Can only be payer once
                    }
                }

                if ($alreadyPayer) {
                    $keptPayer->setAmount($keptPayer->getAmount() + $payer->getAmount());
                    $em->remove($payer);
                } else {
                    $payer->setUser($keptUser);
                }
            }

            foreach($deletedUser->getBeneficiaries() as $beneficiary) {
                $alreadyBeneficiary = false;
                foreach ($beneficiary->getExpenditure()->getBeneficiaries() as $keptBeneficiary) {
                    if ($keptBeneficiary->getUser()->equals($keptUser)) {
                        $alreadyBeneficiary = true;
                        break; // Can only be beneficiary once
                    }
                }

                if ($alreadyBeneficiary) {
                    $keptBeneficiary->setAmount($keptBeneficiary->getAmount() + $beneficiary->getAmount());
                    $em->remove($beneficiary);
                } else {
                    $beneficiary->setUser($keptUser);
                }
            }

            foreach($deletedUser->getAccounts() as $account) {
                $account->removeUser($deletedUser);
                if (!$account->getUsers()->contains($keptUser))
                    $account->addUser($keptUser);
            }

            $em->remove($deletedUser); 
            $em->flush();
            $em->getConnection()->commit();
        } catch (Exception $e) {
            $em->getConnection()->rollback();
            throw $e;
        }
    }

    private function _generateRequestToken(User $user)
    {
        return hash_hmac('sha256', uniqid(mt_rand(), true), $user->getId());
    }

    private function _sendRequest(User $user, $requestToken, User $other)
    {
        $mail = new MergeRequestMail($this->_requester, $other, $user, $this->_getUrlAcceptRequest($requestToken));
        $mail->send();
    }

    private function _getUrlAcceptRequest($token)
    {
        return $this->_account->getUrlAcceptMergeRequest($this, $token);
    }

    public static function getUrlFromIds($uid1, $uid2)
    {
        return Utils::makeUrl('merge-user-'.(int)$uid1.'-with-'.(int)$uid2.'.html');
    }

    public function getUrl()
    {
        return $this->_account->getUrlMergeRequest($this->_firstUser, $this->_secondUser);
    }
}
