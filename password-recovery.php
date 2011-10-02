<?php
/**
 * Fichier de connexion au site
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
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.rendsmoimatune.net
 */

require_once 'inc/init.php';

$em = \Bdf\Core::getInstance()->getEntityManager();
$te = \Bdf\Core::getInstance()->getTemplatesEngine();

if(isset ($_POST['recover-password'])) {
    try {
        $user = Eu\Rmmt\User::getRepository()->findOneBy(array("_email"=>$_POST['email']));

        if (null == $user) {
            throw new Eu\Rmmt\Exception\UserInputException(Bdf\Utils::getText('Unknown email %s', $_POST['email']), $_POST['email'], 'email');
        }

        $user->generatePasswordRecoveryToken();
        $em->flush();

        $recoveryUrl = Bdf\Utils::makeUrl('password-recovery.html?id='.$user->getId().'&token='.$user->getPasswordRecoveryToken());

        $mail = new Eu\Rmmt\Mail\PasswordRecoveryMail($user, $recoveryUrl);
        $mail->send();

        $te->assign("messages", array(array("type"=>"done", "content"=>Bdf\Utils::getText('An email has been sent to you in order to reset your password.'))));
        $te->display('password-recovery');
    } catch(Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign("messages", array(array("type"=>"error", "content"=>$e->getMessage())));
        $te->assign("userInputException", $e);
        $te->assign("_POST", $_POST);
        $te->display("password-recovery");
    } catch(Exception $e) {
        $te->assign("messages", array(array("type"=>"error", "content"=>'Internal error : '.$e->getMessage())));
        $te->assign("_POST", $_POST);
        $te->display("password-recovery");
    }
} elseif (isset ($_GET['token']) OR isset ($_POST['reset-password'])) {
    try {
        $user = Eu\Rmmt\User::getRepository()->find($_REQUEST['id']);

        if (null == $user) {
            throw new Eu\Rmmt\Exception\UnknownUserException($_REQUEST['id']);
        }

        if (!$user->checkPasswordRecoveryToken($_REQUEST['token'])) {
            throw new Eu\Rmmt\Exception\InvalidPasswordRecoveryTokenException($_REQUEST['token']);
        }

        $te->assign('id', $_REQUEST['id']);
        $te->assign('token', $_REQUEST['token']);

        if (isset($_GET['token'])) {
            $te->display("reset-password");
        } elseif (isset($_POST['reset-password'])) {
            if ($_POST['password'] !== $_POST['password-confirm']) {
                throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Password are not identical'), $_POST['password'], 'password');
            }

            $user->setPassword($_POST['password']);
            $user->clearPasswordRecoveryToken();
            $em->flush();

            $messages = array(array("type"=>"done", "content"=>Bdf\Utils::getText("Password successfully changed")));
            \Bdf\Session::getInstance()->add('messages',$messages);
            header("location: ".Bdf\Utils::makeUrl("sign-in.html"));
        }
    } catch (Eu\Rmmt\Exception\UnknownUserException $e) {
        $te->assign("messages", array(array("type"=>"error", "content"=>$e->getMessage())));
        $te->display("error");
    } catch (Eu\Rmmt\Exception\InvalidPasswordRecoveryTokenException $e) {
        $te->assign("messages", array(array("type"=>"error", "content"=>$e->getMessage())));
        $te->display("error");
    } catch (Eu\Rmmt\Exception\UserInputException $e) {
        $te->assign("userInputException", $e);
        $te->assign("messages", array(array("type"=>"error", "content"=>$e->getMessage())));
        $te->display("reset-password");
    } catch (Exception $e) {
        $te->assign("messages", array(array("type"=>"error", "content"=>'Internal error : '.$e->getMessage())));
        $te->display("error");
    }
}else {
    $te->display("password-recovery");
}
?>
