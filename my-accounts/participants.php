<?php
/**
 * Page de gestion des participants
 *
 * PHP version 5.3
 *
 * This file is part of Rendsmoimatune.
 *
 * BotteDeFoin is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * BotteDeFoin is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with BotteDeFoin.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ScriptFile
 * @package  BotteDeFoin
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

$messages = array();

if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}

if (!isset($_GET['account-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    die();
}

$account = \Eu\Rmmt\Account::getRepository()->find($_GET['account-id']);
if (null === $account) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    die();
}

$te->assign('currentAccount', $account);

$users  = array();
$merges = array();

foreach ($account->getUsers() as $user) {
    $name = mb_strtolower($user->getName());
    if (isset($users[$name])) {
        $merges[] = new \Eu\Rmmt\MergeRequest($account, $users[$name], $user, $currentUser); 
        $messages[] = array('type'=>'warning', 'content'=>\Bdf\Utils::getText('Users %1$s and %2$s seems to be the same, you should request a merge.', $users[$name]->getName(), $user->getName()));
    } else {
        $users[$name] = $user;
    }
}


$te->assign('merges', $merges);

if (isset($_POST['merge-users'])) {
    if (isset($_POST['merge']) and sizeof($_POST['merge']) == 2) {
        header('location: '.$account->getUrlMergeRequestFromIds($_POST['merge'][0], $_POST['merge'][1]));
    }
}
//if (isset($_POST['delete-users'])) {
//    try {
//        foreach($_POST['delete'] as $userId => $delete) {
//            try {
//                $user = \Eu\Rmmt\User::getRepository()->find($userId);
//                $user->checkDeleteRight($currentUser);
//                if (null !== $user) {
//                    $user->delete();
//                }
//            } catch(Eu\Rmmt\Exception\RightException $e) {
//                $messages[] = array('type'=>'error','content'=>$e->getMessage());
//            }
//        }
//    } catch (Exception $e) {
//        $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
//        $te->display("error.tpl");
//    }
//    $te->assign('messages', $messages);
//} elseif(isset($_POST['update-users'])) {
//    foreach($_POST['update'] as $userId => $name) {
//        $user = \Eu\Rmmt\User::getRepository()->find($userId);
//        if (null !== $user) {
//            $user->setName($name);
//        }
//    }
//}

$em->flush();

$te->assign('messages', $messages);
$te->display('my-accounts/participants');
?>
