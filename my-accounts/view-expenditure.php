<?php
/**
 * Fichier de modification d'une dépense
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
require_once '../inc/assignDefaultVar.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

$currentUser = \Eu\Rmmt\User::getCurrentUser();
if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}


if (!isset($_GET['expenditure-id']) or empty($_GET['account-id'])) {
    header('location: '.$account->getUrlDetail());
    die();
}

try {
    $expenditure = \Eu\Rmmt\Expenditure::getRepository()->find($_GET['expenditure-id']);
    if (null === $expenditure) {
        header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
        die();
    }

    $expenditure->getEvent()->checkViewRight($currentUser);

    $te->assign("currentAccount",$account);
    $te->assign('expenditure', $expenditure);
    $te->display('my-accounts/view-expenditure');
} catch(Eu\Rmmt\Exception\RightException $e) {
    \Bdf\Session::getInstance()->add('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
}


?>
