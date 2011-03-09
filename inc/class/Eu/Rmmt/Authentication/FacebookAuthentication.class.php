<?php
/**
 * Fichier de classe
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
 * @link     http://www.Rendsmoimatune.fr
 */

namespace Eu\Rmmt\Authentication;
use Doctrine\Common\Collections\ArrayCollection;
use Bdf\Core;
use Bdf\Utils;
use Eu\Rmmt\User;
use Eu\Rmmt\UserFactory;
use Eu\Rmmt\Exception\MergeException;

/**
 * BasicAuthentication
 *
 * @category Class
 * @package  Fr\Rendsmoimatune\Authentication
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.Rendsmoimatune.fr
 */
class FacebookAuthentication extends OAuthentication
{
    private $_clientId     = "129495827110928";
    private $_clientSecret = "f934fd25dd1d7f8b1b39606164b5c09c";
    private $_redirectUrl; 

    public function __construct()
    {
        parent::__construct();
        $this->_redirectUrl = urlencode(Utils::makeUrl('/authentication/facebook.html'));
    }

    protected function _requestRequestToken()
    {
        // Nothing to do with OAuth 2.0
    }

    protected function _constructServiceProviderUrl()
    {
        return "https://graph.facebook.com/oauth/authorize?client_id=".$this->_clientId."&redirect_uri=".$this->_redirectUrl;
    }

    protected function _constructRequestRequestTokenUrl()
    {
        return "";
    }

    protected function _constructRequestAccessTokenUrl()
    {
        return "https://graph.facebook.com/oauth/access_token?client_id=".$this->_clientId."&redirect_uri=".$this->_redirectUrl."&client_secret=".$this->_clientSecret."&code=".$this->_requestToken;
    }

    protected function _constructAccessProtectedRessourcesUrl()
    {
        return "https://graph.facebook.com/me?access_token=".$this->_accessToken;
    }

    protected function _handleProtectedRessources(array $ressources)
    {
        $user = User::getRepository()->findOneBy(array('_facebookId'=>$ressources['id']));
        if (null == $user) {
            $user = UserFactory::createFacebookUser($ressources['id'], $ressources['first_name'].' '.$ressources['last_name']);
            $em = Core::getInstance()->getEntityManager();
            $em->persist($user);
            $em->flush();
        }

        $this->_user = $user;
    }
}
