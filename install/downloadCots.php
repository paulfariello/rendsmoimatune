<?php
/**
 * Fichier d'installation de botteDeFoin
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
 * @category ScriptFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */

define('ROOT', realpath(dirname(__FILE__).'/../').'/');
define('CONFIG_FILE', ROOT.'inc/conf/config.ini');
define('COTS_FILE', ROOT.'install/cots.xml');
define('COTS_DIR', ROOT.'inc/cots/');

header("Cache-Control: no-cache, must-revalidate");
header("Expires: Mon, 28 Nov 1988 05:00:00 GMT");

error_reporting(E_WARNING);
require_once "pclerror.lib.php";
require_once "pcltrace.lib.php";
require_once "pcltar.lib.php";

$config = array();

$config = @parse_ini_file(CONFIG_FILE, true);
if ($config === false) {
    header('location: index.php');
}

$cotsXml = new SimpleXMLElement(COTS_FILE, null, true);

switch($_GET['cots']) {
    case 'doctrine':
        $xpath = "/xml/doctrine/release[@name='".$config['doctrine']['release']."']";
        $extractPath = ROOT.'inc/cots/doctrine/'.$config['doctrine']['release'].'/';
        break;
    case 'templates':
        $xpath = "/xml/templates/engine[@name='".$config['templates']['engine']."']/release[@name='".$config['templates']['release']."']";
        $extractPath = ROOT.'inc/cots/'.$config['templates']['engine'].'/'.$config['templates']['release'].'/';
        break;
    default:
        header('location: index.php');
}

$cots = $cotsXml->xpath($xpath);
if ($cots[0] instanceof SimpleXMLElement) {
    $url = $cots[0]->attributes()->url;
    echo 'Downloading from '.$url.'<br />';
    
    // Trying to conforme filename to pclTar
    $fileName = '/tmp/'.basename(parse_url($url,PHP_URL_PATH));
    if (strrpos($fileName, '.tar.gz') !== strlen($fileName)-strlen('.tar.gz')) {
        $fileName = pathinfo($fileName,PATHINFO_FILENAME).'.tar.gz';
    }

    // Download
    file_put_contents($fileName, file_get_contents($url));

    // Extract
    if (md5_file($fileName) != $cots[0]->attributes()->md5sum) {
        echo "Error while downloading file<br />";
        echo "Expecting md5 sum ".$cots[0]->attributes()->md5sum." but getting ".md5_file($fileName)."<br />";
    }
    echo 'Extracting '.$fileName.' to '.$extractPath.'<br />';
    PclTarExtract($fileName, $extractPath, $cots[0]->attributes()->extractPath);
    echo 'Deleting '.$fileName.'<br />';
    unlink($fileName);

}
?>
<a href="index.php">Retour</a>
