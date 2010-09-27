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

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

$currentUser = \Eu\Rmmt\User::getCurrentUser();
if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}

if (!isset($_GET['event-id'])) {
    header('location: '.\Bdf\Utils::makeUrl('events/'));
    die();
} else {
    $event = $em->getRepository("Eu\Rmmt\Event")->find($_GET['event-id']);
    if ($event === null) {
        header('location: '.\Bdf\Utils::makeUrl('events/'));
        die();
    }
}

if (!isset($_POST['create-new-expenditure'])) {
    $te->assign("currentEvent",$event);
    $te->assign('events',$em->getRepository('Eu\Rmmt\Event')->findAll());
    $te->display('events/create-new-expenditure');
} else {

    try {
        if (!isset($_POST['name']) OR empty($_POST['name'])) {
            throw new Eu\Rmmt\UserInputException(\Bdf\Utils::getText('Name is required'), $_POST['name']);
        }

        if (!isset($_POST['amount']) OR empty($_POST['amount'])) {
            throw new Eu\Rmmt\UserInputException(\Bdf\Utils::getText('Amount is required'), $_POST['amount']);
        }

        $expenditure = new Eu\Rmmt\Expenditure($event, $_POST['name'], $_POST['amount']);

        $date = null;
        if (isset($_POST['date']) AND !empty($_POST['date'])) {
            $date = DateTime::createFromFormat('d-m-Y', $_POST['date']);

            $expenditure->setDate($date);
        }

        // Payers
        $payers = array();
        $payersNameOrId = explode(',', $_POST['payers']);

        $unknown = false;
        foreach($payersNameOrId as $payer) {
            if (ctype_digit($payer)) {
                $user = Eu\Rmmt\User::getRepository()->find((int)$payer);
                if (null !== $user) {
                    $payers[] = $user;
                }
            } else {
                $unknown = true;
                $user = new User(uniqid().'@rendsmoimatune.eu');
                list($firstName, $lastName) = explode(' ', $payer, 2);
                $user->setFirstName($firstName);
                $user->setLastName($lastName);
                $user->setUnknown(true);
                $em->persist($user);
                $payers[] = $user;
            }
        }

        // Involved users
        $involvedNameOrId = explode(',', $_POST['involved']);
        foreach($involvedNameOrId as $involved) {
            if (ctype_digit($involved)) {
                $user = Eu\Rmmt\User::getRepository()->find((int)$involved);
                if (null !== $user) {
                    $involvedUsers[] = $user;
                }
            } else {
                $unknown = true;
                $user = new Eu\Rmmt\User(uniqid().'@rendsmoimatune.eu');
                list($firstName, $lastName) = explode(' ', $involved, 2);
                $user->setFirstName($firstName);
                $user->setLastName($lastName);
                $user->setRegistered(false);
                $involvedUsers[] = $user;
            }
        }

        //TODO calculate amount payed per user, amount due per user
        $amountPerPayers        = $expenditure->getAmount()/count($payers);
        $amountPerInvolvedUsers = $expenditure->getAmount()/count($involvedUsers);

        foreach($payers as $user) {
            $expenditure->addPayingUser($user, $amountPerPayers);
        }

        foreach($involvedUsers as $user) {
            $expenditure->addInvolvedUser($user, $amountPerInvolvedUsers);
        }

        $em->persist($expenditure);
        $em->flush();
        header('location: '.$event->getUrlDetail());
    } catch(Eu\Rmmt\UserInputException $e) {
        $te->assign('currentEvent',$event);
        $te->assign('_POST',$_POST);
        $te->assign('message', array('type'=>'error','content'=>$e->getMessage()));
        $te->display('events/create-new-expenditure');
    } catch(Exception $e) {
        $te->assign('currentEvent',$event);
        $te->assign('_POST',$_POST);
        $te->assign('message', array('type'=>'error','content'=>$e->getMessage()));
        $te->display('events/create-new-expenditure');
    }
}

?>