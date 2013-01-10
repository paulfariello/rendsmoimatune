<?php
/**
 * Configuration checking script
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
 * @category Install
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */

define('HTML_OK', '<span style="color:green">OK</span><br />');
define('HTML_NOK', '<span style="color:red">Erreur</span><br />');
define('HTML_UNSUPPORTED_VERSION', '<span style="color:orange">Version non supportée</span><br />');
define('HTML_COTS_NOT_FOUND', '<span style="color:orange">Fichiers introuvables</span>');

define('ROOT', realpath(dirname(__FILE__).'/../').'/');
define('CONFIG_FILE', ROOT.'inc/conf/config.ini');
define('COTS_FILE', ROOT.'install/cots.xml');
define('COTS_DIR', ROOT.'inc/cots/');

header("Cache-Control: no-cache, must-revalidate");
header("Expires: Mon, 28 Nov 1988 05:00:00 GMT");

$config = array();

function checkConfigFile()
{
    global $config;

    echo 'Vérification du fichier de configuration : ';
    $config = @parse_ini_file(CONFIG_FILE, true);
    if ($config === false) {
        echo HTML_NOK;
        return false;
    } else {
        echo HTML_OK;
        return true;
    }
}

function createConfigFile()
{
    global $config;

    echo '<a href="config.php">Créer le fichier de configuration</a>';
    return false;
}

function checkDirectories()
{
    global $config;

    $return = true;

    /***** Dossier de proxy doctrine *****/
    echo 'Dossier pour les proxy doctine : ';
    if(file_exists(ROOT.$config['doctrine']['proxy_dir']) AND is_dir(ROOT.$config['doctrine']['proxy_dir'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }

    /***** Dossier de mapping doctrine *****/
    echo 'Dossier pour les mapping doctrine : ';
    if(file_exists(ROOT.$config['doctrine']['mapping_dir']) AND is_dir(ROOT.$config['doctrine']['mapping_dir'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }

    /***** Dossier de templates *****/
    echo 'Dossier pour les templates : ';
    if(file_exists(ROOT.$config['templates']['dir'].$config['site']['skin']) AND is_dir(ROOT.$config['templates']['dir'].$config['site']['skin'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }

    /***** Dossier de templates compiles *****/
    if (isset($config['templates']['compiled_dir'])) {
        echo 'Dossier pour les templates compilés : ';
        if(file_exists(ROOT.$config['templates']['compiled_dir']) AND is_dir(ROOT.$config['templates']['compiled_dir']) AND is_writable(ROOT.$config['templates']['compiled_dir'])) {
            echo HTML_OK;
        } else {
            echo HTML_NOK;
            $return = false;
        }
    }

    /***** Dossier de javascript *****/
    echo 'Dossier pour les fichiers javascript : ';
    if(file_exists(ROOT.$config['site']['javascript_dir']) AND is_dir(ROOT.$config['site']['javascript_dir'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }

    /***** Dossier de css *****/
    echo 'Dossier pour les fichiers css : ';
    if(file_exists(ROOT.$config['site']['style_dir'].$config['site']['skin']) AND is_dir(ROOT.$config['site']['style_dir'].$config['site']['skin'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }

    /***** Dossier d'images *****/
    echo 'Dossier pour les images : ';
    if(file_exists(ROOT.$config['site']['image_dir'].$config['site']['skin']) AND is_dir(ROOT.$config['site']['image_dir'].$config['site']['skin'])) {
        echo HTML_OK;
    } else {
        echo HTML_NOK;
        $return = false;
    }
}

function checkCots()
{
    global $config;

    $return = true;

    $cots = new SimpleXMLElement(COTS_FILE, null, true);

    echo 'Vérification de la compatibilité et de la présence de '.$config['templates']['engine'].' : ';
    $supported = false;
    if($cots->xpath("/xml/templates/engine[@name='".$config['templates']['engine']."']/release[@name='".$config['templates']['release']."']")) {
        $supported = true;        
    }

    if ($supported) {
        if ($config['templates']['engine'] != 'simpleTemplatesEngine') {
            $templatesDir = COTS_DIR.$config['templates']['engine'].'/'.$config['templates']['release'];
            if(file_exists($templatesDir) AND is_dir($templatesDir)) {
                echo HTML_OK;
            } else {
                echo HTML_COTS_NOT_FOUND.' <a href="downloadCots.php?cots=templates">Télécharger</a><br />';
                $return = false;
            }
        } else {
            echo HTML_OK;
        }
    } else {
        echo HTML_UNSUPPORTED_VERSION;
        $return = false;
    }

    echo 'Vérification de la compatibilité et de la présence de Doctrine : ';
    $supported = false;
    if($cots->xpath("/xml/doctrine/release[@name='".$config['doctrine']['release']."']")) {
        $supported = true;        
    }

    if ($supported) {
        if(file_exists(COTS_DIR.'doctrine/'.$config['doctrine']['release'])) {
            echo HTML_OK;
        } else {
            echo HTML_COTS_NOT_FOUND.' <a href="downloadCots.php?cots=doctrine">Télécharger</a><br />';
        }
    } else {
        echo HTML_UNSUPPORTED_VERSION;
        $return = false;
    }

    return $return;
}
?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    <title>BotteDeFoin - Installation</title>
</head>
<body>
<?php
if (checkConfigFile()) {
    checkCots();
    checkDirectories();
} else {
    createConfigFile();
}

?>
</body>
</html>
