<?php
/**
 * Fichier de création d'une nouvelle dépense
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

    if (!isset($_POST['create-new-expenditure'])) {
        $te->assign("currentAccount",$account);
        $te->display('my-accounts/create-new-expenditure');
    } else {
        try {
            if (!isset($_POST['title']) OR empty($_POST['title'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Title is required'), $_POST['title'], 'title');
            }

            if (!isset($_POST['amount']) OR empty($_POST['amount'])) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Amount is required'), $_POST['amount'], 'amount');
            }
            $_POST['amount'] = strtr($_POST['amount'], ',', '.');

            $expenditure = new Eu\Rmmt\Expenditure($account, $_POST['title'], $_POST['amount']);

            $date = null;
            if (isset($_POST['date']) AND !empty($_POST['date'])) {
                $date = DateTime::createFromFormat('d-m-Y', $_POST['date']);

                $expenditure->setDate($date);
            }

            // Store new users here in order to propose invitations
            $newUsers       = array();


            // Payers
            $amountPayed    = 0;

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

                    // Definitely unknown user
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

                    $expenditure->addPayer($payer, $amount);
                    $account->addUser($payer);

                    $amountPayed += $amount;
                }
            }

            if ($amountPayed != $expenditure->getAmount()) {
                throw new Eu\Rmmt\Exception\InvalidAmountPayedException($expenditure);
            }

            // Beneficiaries
            $beneficiaries = array();

            foreach( array_keys ($_POST['beneficiariesId']) as $index ) {
                $id   = $_POST['beneficiariesId'][$index];
                $name = trim ($_POST['beneficiariesName'][$index]);

                if (!empty ($name)) {
                    $unknown     = true;
                    $beneficiary = null;

                    // We do know user because he has been auto completed
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

                    $beneficiaries[] = $beneficiary;
                }
            }

            // Calculate amount due per user
            $amountPerBeneficiary = $expenditure->getAmount() / count($beneficiaries);

            foreach($beneficiaries as $beneficiary) {
                $expenditure->addBeneficiary($beneficiary, $amountPerBeneficiary);
                $account->addUser($beneficiary);
            }

            $em->persist($expenditure);
            $em->flush();
            $messages = array();
            $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('Expenditure created'));

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
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
            $te->assign('userInputException', $e);
            $te->display('my-accounts/create-new-expenditure');
        } catch(Exception $e) {
            $te->assign('currentAccount',$account);
            $te->assign('_POST',$_POST);
            $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal Error').' : '.$e->getMessage())));
            $te->display('my-accounts/create-new-expenditure');
        }
    }
} catch(Eu\Rmmt\Exception\RightException $e) {
    \Bdf\Session::getInstance()->add('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    header('location: '.\Bdf\Utils::makeUrl('my-accounts/'));
}

?>
