<?php
/**
 * Page d'affichage des événements
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
if ($currentUser == null) {
    \Bdf\Session::getInstance()->add('redirect',$_SERVER['REQUEST_URI']);
    header('location: '.\Bdf\Utils::makeUrl('sign-in.html'));
    die();
}

if (isset($_POST['create-new-account'])) {

    try {
        if (!isset($_POST['name']) OR empty($_POST['name'])) {
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Title is required'), $_POST['name'], 'title');
        }

        $account = new Eu\Rmmt\Account($_POST['name'], \Eu\Rmmt\User::getCurrentUser());


        // Initialisation des dates
        $startDate = null;
        $endDate   = null;
        if (isset($_POST['start-date']) AND !empty($_POST['start-date']) AND isset($_POST['end-date']) AND !empty($_POST['end-date'])) {
            $startDate = DateTime::createFromFormat('m-d-Y', $_POST['start-date']);
            $endDate   = DateTime::createFromFormat('m-d-Y', $_POST['end-date']);

            // Est-ce que la date de début est inférieur à la date de fin
            if (date_diff($startDate,$endDate)->format('%R') == '-') {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Time period must be positive'), $_POST['end-date'], 'date');
            }

            $account->setStartDate($startDate);
            $account->setEndDate($endDate);
        }

        $em->persist($account);
        $em->flush();
        header('location: '.$account->getUrlDetail());
    } catch(Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->assign('userInputException', $e);
        $te->display('my-accounts/create-new-account');
    } catch(Exception $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
        $te->display('my-accounts/create-new-account');
    }
} else {
    $te->display('my-accounts/create-new-account');
}

?>
