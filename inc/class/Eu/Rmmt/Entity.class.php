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
 * @category ScriptFile
 * @package  Rendsmoimatune
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

namespace Eu\Rmmt;
use Bdf\Core;
use Doctrine\ORM\EntityManager;
use Doctrine\Common\Collections\ArrayCollection;

/**
 * Entity
 *
 * @category Class
 * @package  Eu\Rmmt
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 */
abstract class Entity
{
    /**
     * Get Doctrine repository for current entity
     *
     * @param EntityManager $em Doctrine EntityManager to use. Use default one if none provided.
     *
     * @return Doctrine repository
     */
    public static function getRepository(EntityManager $em = NULL)
    {
        if ($em == NULL)
            $em = Core::getInstance()->getEntityManager();
        return $em->getRepository(get_called_class());
    }
}
