<?php
/**
 * Page de fusion de deux utilisateurs
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

require_once '../inc/init.php';
require_once '../inc/assignDefaultVar.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();
$currentUser = \Eu\Rmmt\User::getCurrentUser();
if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}

try {
    $user1 = \Eu\Rmmt\User::getRepository()->find($_GET['id1']);
    if (null == $user1) {
        throw new \Eu\Rmmt\Exception\UnknownUserException($_GET['id1']);
    }
    $user2 = \Eu\Rmmt\User::getRepository()->find($_GET['id2']);
    if (null == $user2) {
        throw new \Eu\Rmmt\Exception\UnknownUserException($_GET['id2']);
    }

    $query = $em->createQuery( "SELECT mr FROM \Eu\Rmmt\MergeRequest mr INNER JOIN mr._firstUser fu INNER JOIN mr._secondUser su INNER JOIN mr._requester r WHERE r._id = :rid AND ((fu._id = :fuid AND su._id = :suid) OR (fu._id = :suid AND su._id = :fuid))" );
    $query->setParameter("rid", $currentUser->getId());
    $query->setParameter("fuid", $user1->getId());
    $query->setParameter("suid", $user2->getId());
    $query->setMaxResults(1);
    $mergeRequest = $query->execute();

    if (null == $mergeRequest) {
        $mergeRequest = new \Eu\Rmmt\MergeRequest($user1, $user2, $currentUser);
        $em->persist($mergeRequest);
        $em->flush();
    } else {
        $mergeRequest = $mergeRequest[0];
    }

    $te->assign('mergeRequest', $mergeRequest);

    $mergeRequest->checkMergeRight();

    $te->assign('messages', array(array('type'=>'done','content'=>sprintf(\Bdf\Utils::getText('User %s merged with %s'), $user1->getName(), $user2->getName()))));
    $te->display('my-parameters/merge-request');

} catch(Eu\Rmmt\Exception\MergeAuthorizationException $e) {
    $mergeRequest->requestAgreements();
    $em->flush();
    $te->assign('mergeAuthorizationException', $e);
    $te->assign('messages', array(array('type'=>'warning','content'=>$e->getMessage())));
    $te->display('my-parameters/merge-request');
} catch(Exception $e) {
    $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    $te->display('my-parameters/merge-request');
}
?>
