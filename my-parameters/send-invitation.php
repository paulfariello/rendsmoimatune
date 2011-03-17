<?php
/**
 * Page d'envois d'invitation
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

function sendInvitation($email, $userId)
{
    global $currentUser;

    $title = Bdf\Utils::getText('Invition to join Rendsmoimatune');
    $message = "Bonjour %s, ".$currentUser->getName()." vous a invité à rejoindre rendsmoimatune.
        Rendsmoimatune vous permet de savoir en permanance qui vous doit de l'argent, blablabla.
        Pour nous rejoindre cliquez sur le lien suivant : %s";
    $header = '';


    if (!empty($email)) {
        $user = Eu\Rmmt\User::getRepository()->find($userId);
        if (null != $user AND $user->getCreator()->equals($currentUser)) {
            $user->setEmail($email);
            $user->setInvited(true);
            $user->generateInvitationToken();

            mail($email, $title, sprintf($message, $user->getName(), Bdf\Utils::makeUrl('new-account-invitation.html?id='.$user->getId().'&token='.$user->getInvitationToken()))); 
        }
    }

}

if (isset($_POST['send-invitation']) OR isset($_POST['resend-invitation'])) {

    $messages = array();

    if (isset($_POST['resend-invitation']) and isset($_POST['invite'])) {
        foreach($_POST['invite'] as $userId) {
            $email = $_POST['email'][$userId];
            sendInvitation($email, $userId);
        }
        $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::getText('Invitations sended again !'));
    } elseif(isset($_POST['send-invitation'])) {
        foreach($_POST['email'] as $userId => $email) {
            sendInvitation($email, $userId);
        }
        $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::getText('Invitations sended !'));
    }

    $em->flush();

    \Bdf\Session::getInstance()->add('messages',$messages);

    header('location: '.Bdf\Utils::makeUrl('/my-parameters/send-invitation.html'));
} else {
    $query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u INNER JOIN u._creator c WHERE c._id = :userId AND u._registered = false AND u._invited = false");
    $query->setParameter('userId', $currentUser->getId());
    $users = $query->execute();
    $te->assign('users', $users);

    $query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u INNER JOIN u._creator c WHERE c._id = :userId AND u._registered = false AND u._invited = true");
    $query->setParameter('userId', $currentUser->getId());
    $invitedUsers = $query->execute();
    $te->assign('invitedUsers', $invitedUsers);

    $messages = \Bdf\Session::getInstance()->get('messages');
    if (null !== $messages) {
        $te->assign('messages',$messages);
        \Bdf\Session::getInstance()->remove('messages');
    }

    $te->display('my-parameters/send-invitation');
}
?>
