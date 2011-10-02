<?php
/**
 * Page d'acceptation d'une demande de fusion
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
 * @category ScriptFile
 * @package  Rendsmoimatune
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

try {
    $mergeRequest = \Eu\Rmmt\MergeRequest::getRepository()->find($_GET["request"]);

    if (null == $mergeRequest) {
        throw new \Eu\Rmmt\Exception\UnknownMergeRequestException($_GET['request']);
    }

    $te->assign('currentAccount', $mergeRequest->getAccount());
    $te->assign('mergeRequest', $mergeRequest);

    $mergeRequest->acceptMerge($currentUser, $_GET['token']);
    $em->flush();

    $messages[] = array('type'=>'done','content'=>Bdf\Utils::getText('We successfully registered your agreement for the merge.'));

    $mergeRequest->checkMergeRight();

    header('location: '.$mergeRequest->getUrl());

} catch(Eu\Rmmt\Exception\InvalidMergeRequestTokenException $e) {
    $te->assign('invalidMergeRequestTokenException', $e);
    $messages[] = array('type'=>'warning','content'=>$e->getMessage());
    $te->assign('messages', $messages);
    $te->display('merge-request');
} catch(Eu\Rmmt\Exception\UnknownUserException $e) {
    $te->assign('unknownUserException', $e);
    $messages[] = array('type'=>'warning','content'=>$e->getMessage());
    $te->assign('messages', $messages);
    $te->display('merge-request');
} catch(Eu\Rmmt\Exception\MergeAuthorizationException $e) {
    $te->assign('mergeAuthorizationException', $e);
    $requiredAgreement = $e->getRequiredAgreement();
    if (sizeof($requiredAgreement) == 1) {
        $messages[] = array(
            'type'    => 'warning',
            'content' => Bdf\Utils::getText(
                'You still need the agreement of %1$s. For this purpose an email has been sent to %2$s.',
                $requiredAgreement[0]->isRegistered()?$requiredAgreement[0]->getName():$requiredAgreement[0]->getCreator()->getName(),
                $requiredAgreement[0]->isRegistered()?$requiredAgreement[0]->getEmail():$requiredAgreement[0]->getCreator()->getEmail()
            )
        );
    } elseif (sizeof($requiredAgreement) == 2) {
        // Should never happen but never know...
        $messages[] = array(
            'type'    => 'warning',
            'content' => Bdf\Utils::getText(
                'You still need the agreement of %1$s and %2$s. For this purpose email have been sent respectively to %3$s and %4$s.',
                $requiredAgreement[0]->isRegistered()?$requiredAgreement[0]->getName():$requiredAgreement[0]->getCreator()->getName(),
                $requiredAgreement[1]->isRegistered()?$requiredAgreement[1]->getName():$requiredAgreement[1]->getCreator()->getName(),
                $requiredAgreement[0]->isRegistered()?$requiredAgreement[0]->getEmail():$requiredAgreement[0]->getCreator()->getEmail(),
                $requiredAgreement[1]->isRegistered()?$requiredAgreement[1]->getEmail():$requiredAgreement[1]->getCreator()->getEmail()
            )
        );
    } else {
        // Should never happen
    }
    $te->assign('messages', $messages);
    $te->display('merge-request');
} catch(Exception $e) {
    $messages[] = array('type'=>'warning','content'=>$e->getMessage());
    $te->assign('messages', $messages);
    $te->display('error');
}

?>
