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
    private $_firstUser                 = null;
    private $_secondUser                = null;
    private $_firstUserAgreement        = false;
    private $_secondUserAgreement       = false;
    private $_firstUserRequestToken     = null;
    private $_secondUserRequestToken    = null;
    private $_requester                 = null;

    public function  __construct(User $firstUser, User $secondUser, User $requester)
    {
        $this->_firstUser   = $firstUser;
        $this->_secondUser  = $secondUser;
        $this->_requester   = $requester;
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
        if (!$this->_hasFirstUserAgreement() OR !$this->_hasSecondUserAgreement()) {
            throw new MergeAuthorizationException($this);
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
        if (($this->_firstUser->IsRegistered() AND $this->_firstUser->equals($user)) OR $this->_firstUser->getCreator()->equals($user)) {
            if ($token == $this->_firstUserRequestToken) {
                $this->_firstUserAgreement = true; 
            } else {
                throw new InvalidMergeRequestTokenException($token);
            }
        } elseif (($this->_secondUser->IsRegistered() AND $this->_secondUser->equals($user)) OR $this->_secondUser->getCreator()->equals($user)) {
            if ($token == $this->_secondUserRequestToken) {
                $this->_secondUserAgreement = true; 
            } else {
                throw new InvalidMergeRequestTokenException($token);
            }
        } else {
            throw new UnknownUserException($user->getId());
        }
    }

    private function _generateRequestToken(User $user)
    {
        return hash_hmac('sha256', uniqid(mt_rand(), true), $user->getId());
    }

    private function _sendRequest(User $user, $requestToken, User $other)
    {
        $title = Utils::getText('%1$s suggest you merge your account', $this->_requester->getName());
        $message = Utils::getText('Hi %1$s,'."\n"
            .'%2$s suggest you merge your account with %3$s\'s account because he thinks they both are yours.'."\n"
            .'If you think %2$s is right please go to %4$s.', $user->getName(), $this->_requester->getName(), $other->getName(), $this->_getUrlAcceptRequest($requestToken));
        $headers = "From: no-reply@rendsmoimatune.eu\r\n";

        $email = $user->isRegistered() ? $user->getEmail() : $user->getCreator()->getEmail();

        mail($email, $title, $message, $headers); 
    }

    private function _getUrlAcceptRequest($token)
    {
        return Utils::makeUrl('accept-merge.php?request='.$this->_id.'&token='.$token);
    }
}
