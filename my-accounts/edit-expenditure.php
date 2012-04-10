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

if (isset($_GET['account-id']) and !empty($_GET['account-id'])) {
    $account = \Eu\Rmmt\Account::getRepository()->find($_GET['account-id']);
}

if (!isset($_GET['expenditure-id']) or empty($_GET['account-id'])) {
    if (null !== $account) {
        header('location: '.$account->getUrlDetail());
    } else {
        header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
    }
    die();
} else {
    $expenditure = \Eu\Rmmt\Expenditure::getRepository()->find($_GET['expenditure-id']);
    if (null === $expenditure) {
        if (null !== $account) {
            header('location: '.$account->getUrlDetail());
        } else {
            header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
        }
        die();
    }
}

    
try {
    $expenditure->checkEditRight($currentUser);

    if (!isset($_POST['edit-expenditure'])) {
        $te->assign("currentAccount",$account);
        $te->assign('expenditure', $expenditure);
        $te->display('my-accounts/edit-expenditure');
    } else {
        try {
            if (!isset($_POST['title']) OR empty($_POST['title'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Title is required'), $_POST['title'], 'title');
            }

            if (!isset($_POST['amount']) OR empty($_POST['amount'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Amount is required'), $_POST['amount'], 'amount');
            }

            $expenditureAmount = \Bdf\Utils::parseMoneyInput($_POST['amount']);

            $expenditure->setTitle($_POST['title']);
            $expenditure->setAmount($expenditureAmount);

            $date = null;
            if (isset($_POST['date']) AND !empty($_POST['date'])) {
                $date = DateTime::createFromFormat('d-m-Y', $_POST['date']);

                $expenditure->setDate($date);
            }

            // Store new users here in order to propose invitations
            $newUsers       = array();


            // Payers
            $amountPayed    = 0;
            $payers = array();

            foreach( array_keys ($_POST['payersId']) as $index ) {
                $id     = $_POST['payersId'][$index];
                $name   = trim ($_POST['payersName'][$index]);
                $amount = (float) strtr ($_POST['payersAmount'][$index], ',', '.');
                $metric = $_POST['payersMetric'][$index];

                if (!empty($name)) {
                    $unknown = true;
                    $payer   = null;

                    $payer = Eu\Rmmt\User::findByIdOrName($id, $name, $account, $currentUser);

                    if (null == $payer) {
                        // Create new user
                        $payer = Eu\Rmmt\UserFactory::createUnregisteredUser($currentUser, $name);
                    }

                    // Create payer
                    switch ($metric) {
                        case '%':
                            $amount = round($expenditure->getAmount() * $amount / 100, 2);
                            break;
                        case '€':
                            $amount = \Bdf\Utils::parseMoneyInput($amount);
                            break;
                    }

                    $payers[] = new Eu\Rmmt\Payer($expenditure, $payer, $amount);

                    $amountPayed += $amount;
                }
            }

            if ($amountPayed != $expenditure->getAmount()) {
                throw new Eu\Rmmt\Exception\InvalidAmountPayedException($expenditure);
            }

            $expenditure->updatePayers($payers);

            // Beneficiaries
            $beneficiaries = array();

            foreach( array_keys ($_POST['beneficiariesId']) as $index ) {
                $id   = $_POST['beneficiariesId'][$index];
                $name = trim ($_POST['beneficiariesName'][$index]);

                if (!empty ($name)) {
                    $unknown     = true;
                    $beneficiary = null;

                    $beneficiary = Eu\Rmmt\User::findByIdOrName($id, $name, $account, $currentUser);

                    if (null == $beneficiary) {
                        // Create new user
                        $beneficiary = Eu\Rmmt\UserFactory::createUnregisteredUser($currentUser, $name);
                    }

                    $beneficiaries[] = new Eu\Rmmt\Beneficiary($expenditure, $beneficiary, 0);
                }
            }

            // Calculate amount due per user
            $amountPerBeneficiary = $expenditure->getAmount() / count($beneficiaries);

            foreach($beneficiaries as $beneficiary) {
                $beneficiary->setAmount($amountPerBeneficiary);
            }

            $expenditure->updateBeneficiaries($beneficiaries);

            $em->flush();
            $messages = array();
            $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('Expenditure saved'));

            $usersString = "";
            foreach(Eu\Rmmt\UserFactory::getNewUsers() as $index => $user) {
                $usersString .= $user->getName();
                if ($index < sizeof(Eu\Rmmt\UserFactory::getNewUsers())-1) {
                   $usersString .= ', '; 
                }
            }

            if (!empty($usersString)) {
                $messages[] = array('type'=>'info','content'=>Bdf\Utils::nGetText('User %1$s has been created. <a href="%2$s">Invite him ?</a>', 'Users %1$s has been created. <a href="%2$s">Invite them ?</a>', sizeof(Eu\Rmmt\UserFactory::getNewUsers()), $usersString, Bdf\Utils::makeUrl('my-parameters/send-invitation.html')));
            }
            \Bdf\Session::getInstance()->add('messages',$messages);
            header('location: '.$account->getUrlDetail());
        } catch(Eu\Rmmt\Exception\UserInputException $e) {
            $te->assign('currentAccount',$account);
            $te->assign('expenditure',$expenditure);
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
            $te->assign('userInputException', $e);
            $te->display('my-accounts/edit-expenditure');
        } catch(Exception $e) {
            $te->assign('currentAccount',$account);
            $te->assign('expenditure',$expenditure);
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
            $te->display('my-accounts/edit-expenditure');
        }
    }
} catch(Eu\Rmmt\Exception\RightException $e) {
    \Bdf\Session::getInstance()->add('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    header('location: '.$expenditure->getAccount()->getUrlDetail());
}

?>
