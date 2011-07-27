<?php
/**
 * Fichier d'autocompletion d'utilisateur
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
 * @version  SVN: 145
 * @link     http://www.rendsmoimatune.eu
 */

require_once '../inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

header('Content-type: application/json');

$query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u INNER JOIN u._accounts a INNER JOIN a._users uu WHERE ( u._registered = true OR a._id = :aid ) AND uu._id = :id AND LOWER(u._name) LIKE :search");
$query->setParameter('id', Eu\Rmmt\User::getCurrentUser()->getId());
$query->setParameter('aid', $_GET['aid']);
$query->setParameter('search', '%'.strtolower($_GET['q']).'%');
if (!empty($_GET['limit'])) {
    $query->setMaxResults($_GET['limit']);
}
$users = $query->getResult();

$result = array();
foreach($users as $user) {
    $result[] = array("identifier"=>$user->getId(), "value"=>$user->getName());
}
echo json_encode($result);

?>
