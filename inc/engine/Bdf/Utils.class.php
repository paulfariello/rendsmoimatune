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
        $hashLen = array('SHA256' => 64, 'SHA512' => 128);
        $algo = strtoupper(substr($hash, 1, strpos($hash, '}')-1));

        if (!isset($hashLen[$algo])) {
            throw new \Exception("L'algorithme de hashage n'est pas supporté");
        }

        $oldHash = substr($hash, strpos($hash, '}')+1, $hashLen[$algo]);
        $salt = substr($hash, strpos($hash, '}')+$hashLen[$algo]+1, strlen($hash));
        return $oldHash == hash_hmac($algo, $password, $salt);
    }

    /**
     * Protect a string from XSS
     *
     * @param string $str string to protect
     *
     * @return string the protected string
     */
    public static function htmlProtect($str)
    {
        return htmlspecialchars($str);
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
        $path = parse_url(self::makeUrl($url), PHP_URL_PATH);
        if ($path[strlen($path)-1] == "*") {
            return strstr($_SERVER['REQUEST_URI'], substr($path, 0, -1)) !== false;
        } else {
            return $_SERVER['REQUEST_URI'] == $path;
        }
    }

    /**
     * Generate a token for csrf protection
     *
     * @param string $id an identifier for the token
     *
     * @return string
     */
    public static function generateCSRFToken($id)
    {
        $token = hash_hmac('SHA512', mt_rand(), $id);
        Session::getInstance()->storeCSRFToken($id, $token);
        return $token;
    }

    /**
     * Check validity of CSRFToken
     *
     * @param string $id    token identifier
     * @param string $token token
     *
     * @return boolean
     */
    public static function checkCSRFToken($id, $token)
    {
        $session = Session::getInstance();
        if ($session->getCSRFToken($id) == $token) {
            $session->revokeCSRFToken($id);
            return true;
        } else {
            throw new \Exception("Invalid CSRF token");
            return false;
        }
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

    /**
     * Get translation of the string
     */
    public static function getText($str)
    {
        if (func_num_args() > 1) {
            if (is_array(func_get_arg(1))) {
                return vsprintf(getText($str), func_get_arg(1));
            } else {
                return vsprintf(getText($str), array_slice(func_get_args(), 1));
            }
        } else {
            return getText($str);
        }
        
    }

    /**
     * Get plural translation of the string
     */
    public static function nGetText($str1, $str2, $n)
    {
        if (func_num_args() > 3) {
            if (is_array(func_get_arg(3))) {
                return vsprintf(ngetText($str1, $str2, $n), func_get_arg(3));
            } else {
                return vsprintf(ngetText($str1, $str2, $n), array_slice(func_get_args(), 3));
            }
        } else {
            return ngetText($str);
        }
    }

    /**
     * Check if a string has utf7 characters in it
     *
     * By bmorel at ssi dot fr
     *
     * @param  string $string
     * @return boolean $bool
     */
    public static function seemsUtf8($string)
    {
      for ($i = 0; $i < strlen($string); $i++) {
        if (ord($string[$i]) < 0x80) continue; # 0bbbbbbb
        elseif ((ord($string[$i]) & 0xE0) == 0xC0) $n=1; # 110bbbbb
        elseif ((ord($string[$i]) & 0xF0) == 0xE0) $n=2; # 1110bbbb
        elseif ((ord($string[$i]) & 0xF8) == 0xF0) $n=3; # 11110bbb
        elseif ((ord($string[$i]) & 0xFC) == 0xF8) $n=4; # 111110bb
        elseif ((ord($string[$i]) & 0xFE) == 0xFC) $n=5; # 1111110b
        else return false; # Does not match any model
        for ($j=0; $j<$n; $j++) { # n bytes matching 10bbbbbb follow ?
          if ((++$i == strlen($string)) || ((ord($string[$i]) & 0xC0) != 0x80))
          return false;
        }
      }
      return true;
    }

    /**
     * Remove any illegal characters, accents, etc.
     *
     * @param  string $string  String to unaccent
     * @return string $string  Unaccented string
     */
    public static function unaccent($string)
    {
        if ( ! preg_match('/[\x80-\xff]/', $string) ) {
          return $string;
      }

        if (self::seemsUtf8($string)) {
          $chars = array(
          // Decompositions for Latin-1 Supplement
          chr(195).chr(128) => 'A', chr(195).chr(129) => 'A',
          chr(195).chr(130) => 'A', chr(195).chr(131) => 'A',
          chr(195).chr(132) => 'A', chr(195).chr(133) => 'A',
          chr(195).chr(135) => 'C', chr(195).chr(136) => 'E',
          chr(195).chr(137) => 'E', chr(195).chr(138) => 'E',
          chr(195).chr(139) => 'E', chr(195).chr(140) => 'I',
          chr(195).chr(141) => 'I', chr(195).chr(142) => 'I',
          chr(195).chr(143) => 'I', chr(195).chr(145) => 'N',
          chr(195).chr(146) => 'O', chr(195).chr(147) => 'O',
          chr(195).chr(148) => 'O', chr(195).chr(149) => 'O',
          chr(195).chr(150) => 'O', chr(195).chr(153) => 'U',
          chr(195).chr(154) => 'U', chr(195).chr(155) => 'U',
          chr(195).chr(156) => 'U', chr(195).chr(157) => 'Y',
          chr(195).chr(159) => 's', chr(195).chr(160) => 'a',
          chr(195).chr(161) => 'a', chr(195).chr(162) => 'a',
          chr(195).chr(163) => 'a', chr(195).chr(164) => 'a',
          chr(195).chr(165) => 'a', chr(195).chr(167) => 'c',
          chr(195).chr(168) => 'e', chr(195).chr(169) => 'e',
          chr(195).chr(170) => 'e', chr(195).chr(171) => 'e',
          chr(195).chr(172) => 'i', chr(195).chr(173) => 'i',
          chr(195).chr(174) => 'i', chr(195).chr(175) => 'i',
          chr(195).chr(177) => 'n', chr(195).chr(178) => 'o',
          chr(195).chr(179) => 'o', chr(195).chr(180) => 'o',
          chr(195).chr(181) => 'o', chr(195).chr(182) => 'o',
          chr(195).chr(182) => 'o', chr(195).chr(185) => 'u',
          chr(195).chr(186) => 'u', chr(195).chr(187) => 'u',
          chr(195).chr(188) => 'u', chr(195).chr(189) => 'y',
          chr(195).chr(191) => 'y',
          // Decompositions for Latin Extended-A
          chr(196).chr(128) => 'A', chr(196).chr(129) => 'a',
          chr(196).chr(130) => 'A', chr(196).chr(131) => 'a',
          chr(196).chr(132) => 'A', chr(196).chr(133) => 'a',
          chr(196).chr(134) => 'C', chr(196).chr(135) => 'c',
          chr(196).chr(136) => 'C', chr(196).chr(137) => 'c',
          chr(196).chr(138) => 'C', chr(196).chr(139) => 'c',
          chr(196).chr(140) => 'C', chr(196).chr(141) => 'c',
          chr(196).chr(142) => 'D', chr(196).chr(143) => 'd',
          chr(196).chr(144) => 'D', chr(196).chr(145) => 'd',
          chr(196).chr(146) => 'E', chr(196).chr(147) => 'e',
          chr(196).chr(148) => 'E', chr(196).chr(149) => 'e',
          chr(196).chr(150) => 'E', chr(196).chr(151) => 'e',
          chr(196).chr(152) => 'E', chr(196).chr(153) => 'e',
          chr(196).chr(154) => 'E', chr(196).chr(155) => 'e',
          chr(196).chr(156) => 'G', chr(196).chr(157) => 'g',
          chr(196).chr(158) => 'G', chr(196).chr(159) => 'g',
          chr(196).chr(160) => 'G', chr(196).chr(161) => 'g',
          chr(196).chr(162) => 'G', chr(196).chr(163) => 'g',
          chr(196).chr(164) => 'H', chr(196).chr(165) => 'h',
          chr(196).chr(166) => 'H', chr(196).chr(167) => 'h',
          chr(196).chr(168) => 'I', chr(196).chr(169) => 'i',
          chr(196).chr(170) => 'I', chr(196).chr(171) => 'i',
          chr(196).chr(172) => 'I', chr(196).chr(173) => 'i',
          chr(196).chr(174) => 'I', chr(196).chr(175) => 'i',
          chr(196).chr(176) => 'I', chr(196).chr(177) => 'i',
          chr(196).chr(178) => 'IJ',chr(196).chr(179) => 'ij',
          chr(196).chr(180) => 'J', chr(196).chr(181) => 'j',
          chr(196).chr(182) => 'K', chr(196).chr(183) => 'k',
          chr(196).chr(184) => 'k', chr(196).chr(185) => 'L',
          chr(196).chr(186) => 'l', chr(196).chr(187) => 'L',
          chr(196).chr(188) => 'l', chr(196).chr(189) => 'L',
          chr(196).chr(190) => 'l', chr(196).chr(191) => 'L',
          chr(197).chr(128) => 'l', chr(197).chr(129) => 'L',
          chr(197).chr(130) => 'l', chr(197).chr(131) => 'N',
          chr(197).chr(132) => 'n', chr(197).chr(133) => 'N',
          chr(197).chr(134) => 'n', chr(197).chr(135) => 'N',
          chr(197).chr(136) => 'n', chr(197).chr(137) => 'N',
          chr(197).chr(138) => 'n', chr(197).chr(139) => 'N',
          chr(197).chr(140) => 'O', chr(197).chr(141) => 'o',
          chr(197).chr(142) => 'O', chr(197).chr(143) => 'o',
          chr(197).chr(144) => 'O', chr(197).chr(145) => 'o',
          chr(197).chr(146) => 'OE',chr(197).chr(147) => 'oe',
          chr(197).chr(148) => 'R', chr(197).chr(149) => 'r',
          chr(197).chr(150) => 'R', chr(197).chr(151) => 'r',
          chr(197).chr(152) => 'R', chr(197).chr(153) => 'r',
          chr(197).chr(154) => 'S', chr(197).chr(155) => 's',
          chr(197).chr(156) => 'S', chr(197).chr(157) => 's',
          chr(197).chr(158) => 'S', chr(197).chr(159) => 's',
          chr(197).chr(160) => 'S', chr(197).chr(161) => 's',
          chr(197).chr(162) => 'T', chr(197).chr(163) => 't',
          chr(197).chr(164) => 'T', chr(197).chr(165) => 't',
          chr(197).chr(166) => 'T', chr(197).chr(167) => 't',
          chr(197).chr(168) => 'U', chr(197).chr(169) => 'u',
          chr(197).chr(170) => 'U', chr(197).chr(171) => 'u',
          chr(197).chr(172) => 'U', chr(197).chr(173) => 'u',
          chr(197).chr(174) => 'U', chr(197).chr(175) => 'u',
          chr(197).chr(176) => 'U', chr(197).chr(177) => 'u',
          chr(197).chr(178) => 'U', chr(197).chr(179) => 'u',
          chr(197).chr(180) => 'W', chr(197).chr(181) => 'w',
          chr(197).chr(182) => 'Y', chr(197).chr(183) => 'y',
          chr(197).chr(184) => 'Y', chr(197).chr(185) => 'Z',
          chr(197).chr(186) => 'z', chr(197).chr(187) => 'Z',
          chr(197).chr(188) => 'z', chr(197).chr(189) => 'Z',
          chr(197).chr(190) => 'z', chr(197).chr(191) => 's',
          // Euro Sign
          chr(226).chr(130).chr(172) => 'E',
          // GBP (Pound) Sign
          chr(194).chr(163) => '',
          'Ä' => 'Ae', 'ä' => 'ae', 'Ü' => 'Ue', 'ü' => 'ue',
          'Ö' => 'Oe', 'ö' => 'oe', 'ß' => 'ss');

          $string = strtr($string, $chars);
        } else {
          // Assume ISO-8859-1 if not UTF-8
          $chars['in'] = chr(128).chr(131).chr(138).chr(142).chr(154).chr(158)
            .chr(159).chr(162).chr(165).chr(181).chr(192).chr(193).chr(194)
            .chr(195).chr(196).chr(197).chr(199).chr(200).chr(201).chr(202)
            .chr(203).chr(204).chr(205).chr(206).chr(207).chr(209).chr(210)
            .chr(211).chr(212).chr(213).chr(214).chr(216).chr(217).chr(218)
            .chr(219).chr(220).chr(221).chr(224).chr(225).chr(226).chr(227)
            .chr(228).chr(229).chr(231).chr(232).chr(233).chr(234).chr(235)
            .chr(236).chr(237).chr(238).chr(239).chr(241).chr(242).chr(243)
            .chr(244).chr(245).chr(246).chr(248).chr(249).chr(250).chr(251)
            .chr(252).chr(253).chr(255);

          $chars['out'] = "EfSZszYcYuAAAAAACEEEEIIIINOOOOOOUUUUYaaaaaaceeeeiiiinoooooouuuuyy";

          $string = strtr($string, $chars['in'], $chars['out']);
          $doubleChars['in'] = array(chr(140), chr(156), chr(198), chr(208), chr(222), chr(223), chr(230), chr(240), chr(254));
          $doubleChars['out'] = array('OE', 'oe', 'AE', 'DH', 'TH', 'ss', 'ae', 'dh', 'th');
          $string = str_replace($doubleChars['in'], $doubleChars['out'], $string);
        }

        return $string;
    }


    /**
     * Transforme une chaine de caractère en une chaine utilisable dans une url
     */
    public static function urlize($str)
    {
        // Remove all non url friendly characters with the unaccent function
        $str = self::unaccent($str);

        if (function_exists('mb_strtolower'))
        {
            $str = mb_strtolower($str);
        } else {
            $str = strtolower($str);
        }

        // Remove all none word characters
        $str = preg_replace('/\W/', ' ', $str);

        // More stripping. Replace spaces with dashes
        $str = strtolower(preg_replace('/[^A-Z^a-z^0-9^\/]+/', '-',
                           preg_replace('/([a-z\d])([A-Z])/', '\1_\2',
                           preg_replace('/([A-Z]+)([A-Z][a-z])/', '\1_\2',
                           preg_replace('/::/', '/', $str)))));

        return trim($str, '-');
    }

    public static function replace_uri($subject, $replacement)
    {
        // Based on rfc2396
        // URI-reference = [ absoluteURI | relativeURI ] [ "#" fragment ]
        //      absoluteURI   = scheme ":" ( hier_part | opaque_part )
        //      relativeURI   = ( net_path | abs_path | rel_path ) [ "?" query ]
        //
        //      hier_part     = ( net_path | abs_path ) [ "?" query ]
        //      opaque_part   = uric_no_slash *uric
        //
        //      uric_no_slash = unreserved | escaped | ";" | "?" | ":" | "@" |
        //                      "&" | "=" | "+" | "$" | ","
        //
        //      net_path      = "//" authority [ abs_path ]
        //      abs_path      = "/"  path_segments
        //      rel_path      = rel_segment [ abs_path ]
        //
        //      rel_segment   = 1*( unreserved | escaped |
        //                          ";" | "@" | "&" | "=" | "+" | "$" | "," )
        //
        //      scheme        = alpha *( alpha | digit | "+" | "-" | "." )
        //
        //      authority     = server | reg_name
        //
        //      reg_name      = 1*( unreserved | escaped | "$" | "," |
        //                          ";" | ":" | "@" | "&" | "=" | "+" )
        //
        //      server        = [ [ userinfo "@" ] hostport ]
        //      userinfo      = *( unreserved | escaped |
        //                         ";" | ":" | "&" | "=" | "+" | "$" | "," )
        //
        //      hostport      = host [ ":" port ]
        //      host          = hostname | IPv4address
        //      hostname      = *( domainlabel "." ) toplabel [ "." ]
        //      domainlabel   = alphanum | alphanum *( alphanum | "-" ) alphanum
        //      toplabel      = alpha | alpha *( alphanum | "-" ) alphanum
        //      IPv4address   = 1*digit "." 1*digit "." 1*digit "." 1*digit
        //      port          = *digit
        //
        //      path          = [ abs_path | opaque_part ]
        //      path_segments = segment *( "/" segment )
        //      segment       = *pchar *( ";" param )
        //      param         = *pchar
        //      pchar         = unreserved | escaped |
        //                      ":" | "@" | "&" | "=" | "+" | "$" | ","
        //
        //      query         = *uric
        //
        //      fragment      = *uric
        //
        //      uric          = reserved | unreserved | escaped
        //      reserved      = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" |
        //                      "$" | ","
        //      unreserved    = alphanum | mark
        //      mark          = "-" | "_" | "." | "!" | "~" | "*" | "'" |
        //                      "(" | ")"
        //
        //      escaped       = "%" hex hex
        //      hex           = digit | "A" | "B" | "C" | "D" | "E" | "F" |
        //                              "a" | "b" | "c" | "d" | "e" | "f"
        //
        //      alphanum      = alpha | digit
        //      alpha         = lowalpha | upalpha
        //
        //      lowalpha = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" |
        //                 "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" |
        //                 "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
        //      upalpha  = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" |
        //                 "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" |
        //                 "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"
        //      digit    = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" |
        //                 "8" | "9"
        //

        $escaped       = '%[[:xdigit:]][[:xdigit:]]';
        $mark          = '[-_.!~*\'()]';
        $reserved      = '[;/?:@&=+$,]';
        $unreserved    = '([[:alnum:]]|'.$mark.')';
        $pchar         = '('.$unreserved.'|'.$escaped.'|[:@&=+$,])';
        $param         = '('.$pchar.')*';
        $segment       = '('.$pchar.')*'.'(;'.$param.')*';
        $path_segments = $segment.'(/'.$segment.')*';
        $abs_path      = '/'.$path_segments;
        $port          = '[[:digit:]]*';
        $toplabel      = '([[:alpha:]]([[:alnum:]]|-)*[[:alnum:]])|([[:alpha:]])';
        $domainlabel   = '([[:alnum:]]([[:alnum:]]|-)*[[:alnum:]])|([[:alnum:]])';
        $hostname      = '(('.$domainlabel.'\.)*'.$toplabel.'\.?)';
        $IPv4address   = '(todo)';
        $host          = '('.$hostname.'|'.$IPv4address.')';
        $hostport      = $host.'(:'.$port.')?';
        $userinfo      = 'todo';
        $server        = '(('.$userinfo.'@)?'.$hostport.')?';
        $authority     = $server;
        $net_path      = '//'.$authority.'('.$abs_path.')?';
        $uric          = '('.$reserved.'|'.$unreserved.'|'.$escaped.')';
        $query         = '('.$uric.')*';
        $hier_part     = $net_path.'(\?'.$query.')?';
        $scheme        = '[[:alpha:]]([[:alnum:]+-.])*';
        $absoluteURI   = $scheme.':'.$hier_part;
        $pattern       = '#('.$absoluteURI.')#i';

        return preg_replace($pattern, $replacement, $subject);
    }

    public static function getBreadcrumb()
    {
        $numArgs = func_num_args();
        $breadcrumb = \Bdf\Core::getInstance()->getConfig('site', 'title');
        for ($i = 0; $i < $numArgs; $i++) {
            $breadcrumb .= " – ".func_get_arg($i); 
        }

        return $breadcrumb;
    }
}
