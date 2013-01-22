<?php
/**
 * OAuth resource owner autorization end point as defined in 2.2. Resource Owner Authorization
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
 * @package  Rendsmoimatune
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
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

try {
    $oauth = new Eu\Rmmt\Api\OAuth($em);
    if (isset($_GET['oauth_token'])) {
        $requestToken = $oauth->getRequestToken();
        $te->assign('token', $requestToken);
        $te->assign('consumer', $requestToken->getConsumer());
        $te->display('api/oauth-authorization');
    } else if (isset($_POST['oauth_token'])) {
        if (!Bdf\Utils::checkCSRFToken('oauth-authorization',$_POST['csrf-token']))
            throw new Eu\Rmmt\Exception\UserInputException(\Bdf\Utils::getText('Invalid CSRF token'), $_POST['csrf-token'], 'csrf-token');

        $requestToken = $oauth->getRequestToken();
        $te->assign('consumer', $requestToken->getConsumer());

        if (isset($_POST['allow-access'])) {
            $oauth->grantAccess($currentUser, $requestToken);
            $te->assign('callback', $requestToken->getCallback());
            $te->display('api/access-allowed');
        } else {
            $oauth->denyAccess($currentUser, $requestToken);
            $te->assign('callback', $requestToken->getCallback());
            $te->display('api/access-denied');
        }
    } else {
        throw new \Eu\Rmmt\Exception\OAuthException(400, "Missing required parameter : oauth_token");
    }
} catch (Eu\Rmmt\Exception\UserInputException $e) {
    $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    $te->display('error');
} catch (Eu\Rmmt\Exception\OAuthException $e) {
    $te->assign('messages', array(array('type'=>'error','content'=>$e->getMessage())));
    $te->display('error');
}

?>
