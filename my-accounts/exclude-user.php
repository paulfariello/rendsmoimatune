<?php
/**
 * User exclusion page
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
 * @link     http://www.rendsmoimatune.eu
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

if (!isset($_GET['account-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    die();
}

try {
    $account = \Eu\Rmmt\Account::getRepository()->find($_GET['account-id']);
    if (null === $account) {
        header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
        die();
    }
    $te->assign('currentAccount', $account);


    $user = \Eu\Rmmt\User::getRepository()->find($_GET['id']);
    if (null == $user) {
        header('location: '.$account->getUrlParticipants());
        die();
    }

    $te->assign('user', $user);

    $account->checkExcludeRight($currentUser, $user);

    if (isset($_POST['confirm-exclusion'])) {
        Bdf\Utils::checkCSRFToken('confirm-exclusion', $_POST['csrf-token']);

        $username = $user->getName();

        $account->removeUser($user);
        if ($user->isRegistered())
            $em->remove($user);

        $em->flush();

        \Bdf\Session::getInstance()->add('messages', array(array('type'=>'done','content'=>sprintf(\Bdf\Utils::getText('User %1$s successfully excluded from %2$s'), $username, $account->getName()))));
        header('location: '.$account->getUrlParticipants());
    } else {
        $te->display('my-accounts/exclude');
    }
} catch(\Eu\Rmmt\Exception\ExcludeConstrainedException $e) {
    $te->assign('expenditures', $e->getExpenditures());
    $te->assign('repayments', $e->getRepayments());
    $te->display('my-accounts/exclude-constrained');
} catch(Exception $e) {
    $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    $te->display('error');
}
?>
