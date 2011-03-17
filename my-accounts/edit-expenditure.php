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
    $account = \Eu\Rmmt\Event::getRepository()->find($_GET['account-id']);
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
        $_POST['amount'] = strtr($_POST['amount'], ',', '.');

        $expenditure->setTitle($_POST['title']);
        $expenditure->setAmount($_POST['amount']);

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
            $amount = (float) strtr($_POST['payersAmount'][$index], ',', '.');
            $metric = $_POST['payersMetric'][$index];

            if (!empty($name)) {
                $unknown = true;
                $payer   = null;

                // Get user
                if (!empty($id) and ctype_digit ($id)) {
                    $user = Eu\Rmmt\User::getRepository()->find((int)$id);

                    // Check inconsistency between id and name
                    if (null !== $user and $user->getName() == $name) {
                        $unknown     = false;
                        $payer       = $user;
                    }
                }

                // Search for similar user name
                if ($unknown) {
                    $query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u WHERE LOWER(u._name) = :search");
                    $query->setParameter('search',strtolower($name));
                    $users = $query->getResult();
                    if (!empty($users)) {
                        $unknown    = false;
                        $payer      = $users[0];
                    }
                }

                if ($unknown) {
                    // Create new user
                    $user = Eu\Rmmt\UserFactory::createUnregisteredUser($currentUser, $name);

                    $payer       = $user;
                    $newUsers[]  = $payer;
                }

                // Create payer
                switch ($metric) {
                    case '%':
                        $amount = round($expenditure->getAmount() * $amount / 100, 2);
                        break;
                    case '€':
                        $amount = $amount;
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

                if (!empty ($id) and ctype_digit ($id)) {
                    $user = Eu\Rmmt\User::getRepository()->find((int)$id);

                    // Check inconsistency between id and name
                    if (null !== $user and $user->getName() == $name) {
                        $unknown     = false;
                        $beneficiary = $user;
                    }
                }

                // Search in just created user
                if ($unknown) {
                    foreach($newUsers as $user) {
                        if (strtolower($user->getName()) == strtolower($name)) {
                            $unknown     = false;
                            $beneficiary = $user;
                            break;
                        }
                    }
                }

                // Search for similar user name
                if ($unknown) {
                    $query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u WHERE LOWER(u._name) = :search");
                    $query->setParameter('search',strtolower($name));
                    $users = $query->getResult();
                    if (!empty($users)) {
                        $unknown        = false;
                        $beneficiary    = $users[0];
                    }
                }

                if ($unknown) {
                    // Create new user
                    $user = Eu\Rmmt\UserFactory::createUnregisteredUser($currentUser, $name);

                    $beneficiary = $user;
                    $newUsers[]  = $beneficiary;
                }

                $beneficiaries[] = new Eu\Rmmt\Beneficiary($expenditure, $beneficiary, 0);
            }
        }

        // Calculate amount due per user
        $amountPerBeneficiary = round($expenditure->getAmount() / count($beneficiaries), 2);

        $amountOwed = $expenditure->getAmount();
        foreach($beneficiaries as $index=>$beneficiary) {
            $amountOwed = round($amountOwed - $amountPerBeneficiary, 2);
            $beneficiary->setAmount($amountPerBeneficiary);
        }

        #We let the last guy take the remaining amount owed it for its own. It happens when total amount isn't divisible by number of beneficiaries.
        #Note that remaining amount can be negative.
        $beneficiary->setAmount($amountPerBeneficiary + $amountOwed);

        $expenditure->updateBeneficiaries($beneficiaries);

        $em->flush();
        $messages = array();
        $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('Expenditure saved'));
        $usersString = "";
        foreach($newUsers as $index => $user) {
            $usersString .= $user->getName();
            if ($index < sizeof($newUsers)-1) {
               $usersString .= ', '; 
            }
        }
        if (!empty($usersString)) {
            $messages[] = array('type'=>'info','content'=>Bdf\Utils::getText('User %1$s has been created. <a href="%2$s">Invite them ?</a>', $usersString, Bdf\Utils::makeUrl('my-parameters/send-invitation.html')));
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

?>
