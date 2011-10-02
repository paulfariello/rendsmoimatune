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
class PasswordRecoveryMail extends Mail
{
    public function  __construct(User $to, $recoveryPageUrl)
    {
        $subject = Utils::getText('Password recovery').' - '.Core::getInstance()->getConfig("site", "site_name");
        $message = Utils::getText("You asked to reset your password. To do so, please click this link:\n%1\$s\nThis will let you change your password to something new. If you didn't ask for this, don't worry, we'll keep your password safe.\nBest regards, %2\$s.", $recoveryPageUrl, Core::getInstance()->getConfig("site", "site_name"));

        parent::__construct($to, $subject, $message);
    }
}
