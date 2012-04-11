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

$query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u INNER JOIN u._creator c WHERE c._id = :userId AND u._registered = false AND u._invited = false");
$query->setParameter('userId', $currentUser->getId());
$users = $query->execute();
$te->assign('users', $users);

$query = $em->createQuery("SELECT u FROM Eu\Rmmt\User u INNER JOIN u._creator c WHERE c._id = :userId AND u._registered = false AND u._invited = true");
$query->setParameter('userId', $currentUser->getId());
$invitedUsers = $query->execute();
$te->assign('invitedUsers', $invitedUsers);

if (isset($_POST['send-invitation']) OR isset($_POST['resend-invitation'])) {
    try {
        $messages = array();
        $sended = array();
        $merged = array();
        $mergeRequested = array();

        $invitations = array();

        if (isset($_POST['resend-invitation']) and isset($_POST['invite'])) {
            foreach($_POST['invite'] as $userId) {
                $email = $_POST['email'][$userId];
                $invitations[$userId] = $email;
            }
        } elseif(isset($_POST['send-invitation'])) {
            $invitations = $_POST['email'];
        }

        foreach($invitations as $userId => $email) {
            if (!empty($email)) {
                $user = Eu\Rmmt\User::getRepository()->find($userId);
                if (null == $user OR $user->isRegistered()) {
                    continue;
                }

                $invitedUser = Eu\Rmmt\User::getRepository()->findOneBy(array('_email'=>$email));
                if ($invitedUser != null) {
                    // It's a merge, baby !
                    $query = $em->createQuery( "SELECT mr FROM \Eu\Rmmt\MergeRequest mr INNER JOIN mr._firstUser fu INNER JOIN mr._secondUser su INNER JOIN mr._requester r WHERE (fu._id = :fuid AND su._id = :suid) OR (fu._id = :suid AND su._id = :fuid)" );
                    $query->setParameter("fuid", $user->getId());
                    $query->setParameter("suid", $invitedUser->getId());
                    $query->setMaxResults(1);
                    $mergeRequest = $query->execute();

                    if (null == $mergeRequest) {
                        $accounts = $user->getAccounts();
                        if ($accounts->count() < 1) {
                            throw new Exception("Euuh je m'en doutais mais c'est chiant quand même...");
                        }
                        $mergeRequest = new \Eu\Rmmt\MergeRequest($accounts->first(), $user, $invitedUser, $currentUser);
                        $em->persist($mergeRequest);
                        $em->flush();
                    } else {
                        $mergeRequest = $mergeRequest[0];
                    }

                    try {
                        $mergeRequest->checkMergeRight();
                        $merged[$userId] = $email;
                        // TODO is it to currentUser to do the merge ? Don't think so...
                        $mergeRequest->doMerge(); 
                    } catch(Eu\Rmmt\Exception\MergeAuthorizationException $e) {
                        $mergeRequest->requestAgreements();
                        $mergeRequested[$userId] = $email;
                        $em->flush();
                    }


                } else {
                    $user->sendInvitation($email);
                    $sended[$userId] = $email;
                }
            }
        }


        if (!empty($sended)) {
            $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::nGetText('Invitation sended to %1$s.', 'Invitations sended to %1$s.', sizeof($sended), implode(', ', $sended)));
        }
        if (!empty($merged)) {
            $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::nGetText('%1$s is already on %2$s so we merged his account with your user.', '%1$s are already on %2$s so we merged their account with your users.', sizeof($merged), implode(', ', $merged), Bdf\Core::getInstance()->getConfig("site","site_name")));
        }
        if (!empty($mergeRequested)) {
            $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::nGetText('%1$s is already on %2$s so we proposed him to merged his account with your user.', '%1$s are already on %2$s so we proposed them to merged their account with your users.', sizeof($mergeRequested), implode(', ', $mergeRequested), Bdf\Core::getInstance()->getConfig("site", "site_name")));
        }

        \Bdf\Session::getInstance()->add('messages',$messages);
        
        header('location: '.Bdf\Utils::makeUrl('/my-parameters/send-invitation.html'));
    } catch(Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->assign('userInputException', $e);
        $te->display('my-parameters/send-invitation');
    } catch(Exception $e) {
        $te->assign('_POST',$_POST);
        echo $e->getTraceAsString();
        $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
        $te->display('my-parameters/send-invitation');
    }
} elseif(isset($_POST['send-invitation-to-new-user'])) {
    try {
        if (!isset($_POST['email']) or empty($_POST['email'])) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText("Email is required"), $_POST['email'], 'email');
        } 

        if (!isset($_POST['name']) or empty($_POST['name'])) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText("Name is required"), $_POST['name'], 'name');
        } 
        
        $user = Eu\Rmmt\User::getRepository()->findOneBy(array('_email'=>$_POST['email']));
        if ($user != null) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText("Email %s is already registered.", $_POST['email']), $_POST['email'], 'email');
        }

        $user = Eu\Rmmt\UserFactory::createUnregisteredUser($currentUser, $_POST['name']);
        $user->sendInvitation($_POST['email']);
        $em->persist($user);

        $em->flush();

        $messages[] = array('type'=>'info', 'content'=>Bdf\Utils::getText('Invitation sended !'));
        \Bdf\Session::getInstance()->add('messages',$messages);
        
        header('location: '.Bdf\Utils::makeUrl('/my-parameters/send-invitation.html'));
    } catch(Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
        $te->assign('userInputException', $e);
        $te->display('my-parameters/send-invitation');
    } catch(Exception $e) {
        $te->assign('_POST',$_POST);
        $te->assign('messages', array(array('type'=>'error','content'=>Bdf\Utils::getText('Internal error').' : '.$e->getMessage())));
        $te->display('my-parameters/send-invitation');
    }

} else {
    $messages = \Bdf\Session::getInstance()->get('messages');
    if (null !== $messages) {
        $te->assign('messages',$messages);
        \Bdf\Session::getInstance()->remove('messages');
    }

    $te->display('my-parameters/send-invitation');
}
?>
