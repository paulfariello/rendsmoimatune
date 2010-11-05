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

namespace Eu\Rmmt;
use Doctrine\Common\Collections\ArrayCollection;
use Bdf\Core;

/**
 * UserFactory
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\User
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class UserFactory
{
    /**
     * Create
     *
     */
    public static function createUnregisteredUser(User $creator, $firstName, $lastName)
    {
        $user = new Eu\Rmmt\User(uniqid().'@rendsmoimatune.eu');
        $user->setRegistered(false);
        $user->setFirstName($firstName);
        $user->setLastName($lastName);
        $user->setCreator($creator);
        return $user;
    }
}
