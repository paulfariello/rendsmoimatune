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

$search = '%'.$_GET['q'].'%';

$query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u WHERE u._firstName LIKE :search OR u._lastName LIKE :search OR u._email LIKE :search");
if (!empty($_GET['limit'])) {
    $query->setMaxResults($_GET['limit']);
}
$users = $query->execute(array('search'=>$search));

$result = array();
foreach($users as $user) {
    $result[] = array("identifier"=>$user->getId(), "value"=>$user->getName());
}
echo json_encode($result);

?>