<?php
/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of BotteDeFoin.
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
 * @category ClassFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

namespace Bdf;

/**
 * Utils
 *
 * Une collection de fonctions utiles pour le développement de site web
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */
class Utils
{
    const RESSOURCE_IMAGE = 1;
    const RESSOURCE_TEMPLATE = 2;
    const RESSOURCE_CSS = 3;

    private static $_instance = null;



    /**
     * Constructeur
     *
     * @return Utils
     */
    private function __construct()
    {

    }

    /**
     * Construit une url
     *
     * @param string $url  L'url relative
     * @param string $type Le type d'url
     *
     * @return string
     */
    public static function makeUrl($url,$type = null)
    {
        if (isset($url[0]) AND $url[0] == '/') {
            $url = substr($url, 1);
        }
        $core = \Bdf\Core::getInstance();
        switch($type) {
            case "js":
                return $core->getConfig('site', 'url').$core->getConfig('site', 'javascript_dir').$core->getConfig('site', 'skin').'/'.$url;
                break;
            case "css":
                return $core->getConfig('site', 'url').$core->getConfig('site', 'style_dir').$core->getConfig('site', 'skin').'/'.$url;
                break;
            case "img":
                return $core->getConfig('site', 'url').$core->getConfig('site', 'image_dir').$core->getConfig('site', 'skin').'/'.$url;
                break;
            default:
                return \Bdf\Core::getInstance()->getConfig('site', 'url').$url;
        }
    }

    /**
     * Hash un mot de passe
     *
     * @param string $password Le mot de passe
     *
     * @return string
     */
    public static function hashPassword($password)
    {
        $algo = \Bdf\Core::getInstance()->getConfig("sgbd", "hash");
        $salt = uniqid(mt_rand(), false);
        $hash = hash_hmac($algo, $password, $salt);
        return '{'.$algo.'}'.$hash.$salt;
    }

    /**
     * Compare qu'un mot de passe correspond bien au hash
     *
     * @param string $password Le password
     * @param string $hash     Le hash
     *
     * @return boolean
     */
    public static function comparePassword($password, $hash)
    {
        $hashLen = array('SHA256' => 64);
        $algo = substr($hash, 1, strpos($hash, '}')-1);

        if (!isset($hashLen[$algo])) {
            throw new \Exception("L'algorithme de hashage n'est pas supporté");
        }

        $oldHash = substr($hash, strpos($hash, '}')+1, $hashLen[$algo]);
        $salt = substr($hash, strpos($hash, '}')+$hashLen[$algo]+1, strlen($hash));
        return $oldHash == hash_hmac($algo, $password, $salt);
    }

    /**
     * Est-ce que l'url correspond à la page courante
     *
     * @param string $url L'url à tester
     *
     * @return boolean
     */
    public static function isCurrentPage($url)
    {
        return strstr($url, $_SERVER['REQUEST_URI']) !== false;
    }

    /**
     * Transforme un entier en taille d'octet
     */
    public static function intToByteQuantity($int)
    {
        $suffix = array("Octet","Kio","Mio","Gio","Tio","Pio","Eio","Zio","Yio");
        $i = 0;
        while($int >= 1024) {
            $int = $int/1024;
            $i++;
        }
        return round($int,2)." ".$suffix[$i];
    }
}
