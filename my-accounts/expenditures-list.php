<?php
/**
 * Page listant les dépenses liées à un événement.
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

if (!isset($_GET['account-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    die();
}

if (isset($_GET['page'])) {
    $page = (int)$_GET['page'];
} else {
    $page = 1;
}


$account = Eu\Rmmt\Account::getRepository()->find($_GET['account-id']);

$te->assign("page", $page);
$te->assign("lastPage", ceil($account->getExpenditures()->count() / Bdf\Core::getInstance()->getConfig('rmmt', 'expenditure_limit_number')));
$te->assign('currentAccount', $account);
$te->assign('expenditures', $account->getExpenditures(Bdf\Core::getInstance()->getConfig('rmmt', 'expenditure_limit_number'), $page)); 
$messages = \Bdf\Session::getInstance()->get('messages');
if (null !== $messages) {
    $te->assign('messages',$messages);
    \Bdf\Session::getInstance()->remove('messages');
}
$te->display('my-accounts/expenditures-list');

?>
