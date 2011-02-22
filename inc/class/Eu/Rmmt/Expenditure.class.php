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
    private $_title;
    private $_date;
    private $_amount;
    private $_payers;
    private $_beneficiaries;
    private $_tags;
    private $_event;

    public function __construct(Event $event, $title, $amount)
    {
        $this->_event          = $event;
        $this->_title          = $title;
        $this->_amount         = $amount;
        $this->_payers         = new ArrayCollection();
        $this->_beneficiaries  = new ArrayCollection();
        $this->_tags           = new ArrayCollection();
    }

    public function getId()
    {
        return $this->_id;
    }

    public function getTitle()
    {
        return $this->_title;
    }

    public function setTitle($title)
    {
        $this->_title = $title;
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
        // On vérifie que le payeur n'existe pas deja
        $payer = null;
        foreach ($this->_payers as $tempPayer) {
            if ($tempPayer->getUser()->getId() == $user->getId()) {
                $payer = $tempPayer;        
            }
        }

        if (null == $payer) {
            $payer = new Payer($this, $user, $amount);
        } else {
            $payer->setAmount($amount);
        }

        $this->_payers->add($payer);
    }

    public function removePayer(User $user)
    {
        //TODO remove paying user
    }

    public function removePayers()
    {
        $em = Core::getInstance()->getEntityManager();
        foreach($this->_payers as $payer) {
            $em->remove($payer);
        }
    }


    public function updatePayers(array $payers)
    {
        $em = Core::getInstance()->getEntityManager();
        // On met à jour les ancien payeurs
        foreach($this->_payers as $oldPayer) {
            $amount = null;
            // On cherche si l'ancien payeur existe toujours
            foreach($payers as $index => $newPayer) {
                if ($newPayer->getUser()->getId() == $oldPayer->getUser()->getId()) {
                    $amount = $newPayer->getAmount();
                    unset($payers[$index]);
                    break;
                }
            }

            // On met a jour le montant ou on supprime le payeur
            if (null == $amount) {
                $em->remove($oldPayer);
            } else {
                $oldPayer->setAmount($amount);
            }
        }

        foreach($payers as $newPayer) {
            $this->_payers->add($newPayer);
            $this->_event->addUser($newPayer->getUser());
        }
    }

    public function getBeneficiaries()
    {
        return $this->_beneficiaries;
    }

    public function addBeneficiary(User $user, $amount)
    {
        // On vérifie que le beneficiare n'existe pas deja
        $beneficiary = null;
        foreach ($this->_beneficiaries as $tempBeneficiary) {
            if ($tempBeneficiary->getUser()->getId() == $user->getId()) {
                $beneficiary = $tempBeneficiary;        
            }
        }

        if (null == $beneficiary) {
            $beneficiary = new Beneficiary($this, $user, $amount);
        } else {
            $beneficiary->setAmount($amount);
        }

        $this->_beneficiaries->add($beneficiary);
        $this->_event->addUser($beneficiary->getUser());
    }

    public function removeBeneficiary(User $user)
    {
        //TODO remove involved user
    }

    public function removeBeneficiaries()
    {
        $em = Core::getInstance()->getEntityManager();
        foreach($this->_beneficiaries as $beneficiary) {
            $em->remove($beneficiary);
        }

        $this->_beneficiaries->clear();
    }

    public function updateBeneficiaries(array $beneficiaries)
    {
        $em = Core::getInstance()->getEntityManager();
        // On met à jour les ancien beneficiaires
        foreach($this->_beneficiaries as $oldBeneficiary) {
            $amount = null;
            // On cherche si l'ancien beneficiaire existe toujours
            foreach($beneficiaries as $index => $newBeneficiary) {
                if ($newBeneficiary->getUser()->getId() == $oldBeneficiary->getUser()->getId()) {
                    $amount = $newBeneficiary->getAmount();
                    unset($beneficiaries[$index]);
                    break;
                }
            }

            // On met a jour le montant ou on supprime le payeur
            if (null == $amount) {
                $em->remove($oldBeneficiary);
            } else {
                $oldBeneficiary->setAmount($amount);
            }
        }

        foreach($beneficiaries as $newBeneficiary) {
            $beneficiary = new Beneficiary($this, $newBeneficiary->getUser(), $newBeneficiary->getAmount());
            $this->_beneficiaries->add($beneficiary);
            $this->_event->addUser($beneficiary->getUser());
        }
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

    public function getEvent()
    {
        return $this->_event;
    }

    public function setEvent($event)
    {
        $this->_event = $event;
    }

    public static function getRepository()
    {
        return \Bdf\Core::getInstance()->getEntityManager()->getRepository(__CLASS__);
    }

    public function getUrlDelete()
    {
        return $this->_event->getUrlDeleteExpenditure($this);
    }

    public function getUrlEdit()
    {
        return $this->_event->getUrlEditExpenditure($this);
    }
}
