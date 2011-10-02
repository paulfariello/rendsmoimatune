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
 * Mail for invitation
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class InvitationMail extends Mail
{
    public function  __construct(User $from, User $to, $invitationUrl)
    {
        $subject = Utils::getText('Invitation to join %1$s', Core::getInstance()->getConfig("site", "site_name"));
        $message = Utils::getText('Bonjour %2$s,'."\n\n".'%3$s vous invite à rejoindre %1$s.'."\n".'%1$s vous permet de savoir en permanance qui vous doit de l\'argent.'."\n\n".'Pour rejoindre %1$s cliquez sur le lien suivant :'."\n\n".' %4$s'."\n\n".'Cordialement,', Core::getInstance()->getConfig("site", "site_name"), $to->getName(), $from->getName(), $invitationUrl);

        parent::__construct($to, $subject, $message);
    }
}
