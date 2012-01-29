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

namespace Eu\Rmmt\Mail;
use Eu\Rmmt\User;
use Bdf\Core;
use Bdf\Utils;

/**
 * Mail for password recovery
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class MergeRequestMail extends Mail
{
    public function  __construct(User $requester, User $userToMerge, User $requestedUser, $acceptRequestUrl)
    {
        if ($requestedUser->isRegistered()) {
            $subject = Utils::getText('%1$s suggest you merge your account', $requester->getName());
            $message = Utils::getText('Hi %1$s,'."\n"
                .'%2$s suggest you merge your account with %3$s\'s account because he thinks they both are yours.'."\n"
                .'If you think %2$s is right please go to %4$s .', $requestedUser->getName(), $requester->getName(), $userToMerge->getName(), $acceptRequestUrl);

            $to = $requestedUser;
        } else {
            $subject = Utils::getText('%1$s suggest you merge an account you have created', $requester->getName());
            $message = Utils::getText('Hi %1$s,'."\n"
                .'%2$s suggest you merge %3$s\'s account with %4$s\'s account because he thinks they both are yours.'."\n"
                .'If you think %2$s is right please go to %5$s .', $requestedUser->getCreator()->getName(), $requester->getName(), $userToMerge->getName(), $requestedUser->getName(), $acceptRequestUrl);

            $to = $requestedUser->getCreator();
        }

        parent::__construct($to, $subject, $message);
    }
}
