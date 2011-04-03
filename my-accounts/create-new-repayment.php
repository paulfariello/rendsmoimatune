<?php
/**
 * Fichier de création d'un nouveau remboursement
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

if (!isset($_GET['account-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    die();
} else {
    $account = \Eu\Rmmt\Account::getRepository()->find($_GET['account-id']);
    if ($account === null) {
        header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
        die();
    }
}

try {
    $account->checkCreateRight($currentUser);

    if (!isset($_POST['create-new-repayment'])) {
        $te->assign("currentAccount",$account);
        $te->display('my-accounts/create-new-repayment');
    } else {
        try {
            if (!isset($_POST['payerId']) OR empty($_POST['payerId'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Payer\'s name is required'), $_POST['amount']);
            }
            $payer = Eu\Rmmt\User::getRepository()->find($_POST['payerId']);

            if (!isset($_POST['beneficiaryId']) OR empty($_POST['beneficiaryId'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Beneficiary\'s is required'), $_POST['amount']);
            }
            $beneficiary = Eu\Rmmt\User::getRepository()->find($_POST['beneficiaryId']);


            if (!isset($_POST['amount']) OR empty($_POST['amount'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Amount is required'), $_POST['amount']);
            }

            $repayment = new Eu\Rmmt\Repayment($account, $payer, $beneficiary, $_POST['amount']);

            $date = null;
            if (isset($_POST['date']) AND !empty($_POST['date'])) {
                $date = DateTime::createFromFormat('d-m-Y', $_POST['date']);

                $repayment->setDate($date);
            }

            $em->persist($repayment);
            $em->flush();
            $messages = array();
            $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('Repayment created'));
            \Bdf\Session::getInstance()->add('messages',$messages);
            header('location: '.$account->getUrlDetail());
        } catch(Eu\Rmmt\Exception\UserInputException $e) {
            $te->assign('currentAccount',$account);
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
            $te->display('my-accounts/create-new-repayment');
        } catch(Exception $e) {
            $te->assign('currentAccount',$account);
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
            $te->display('my-accounts/create-new-repayment');
        }
    }
} catch(Eu\Rmmt\Exception\RightException $e) {
    \Bdf\Session::getInstance()->add('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
}

?>
