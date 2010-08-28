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
 * @category ClassFile
 * @package  BotteDeFoin
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: 145
 * @link     http://www.bottedefoin.net
 */

define('HTML_OK', '<span style="color:green">OK</span><br />');
define('HTML_NOK', '<span style="color:red">Erreur</span><br />');
define('CONFIG_FILE', dirname(__FILE__).'/../inc/conf/config.ini');
define('TEMPLATES_COMPILED_DIR', dirname(__FILE__).'/../templates_c/');
define('TEMPLATES_COMPILED_DIR_MOD', 0770);

$config = array();

function checkConfigFile()
{
    global $config;

    echo 'Vérification du fichier de configuration : ';
    $config = parse_ini_file(CONFIG_FILE, true);
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

    echo 'Creation du fichier de configuration : ';
    //TODO copier config.ini.dist

    //TODO configurer config.ini
    
    echo HTML_NOK;
    return false;
}

function checkTemplatesDir()
{
    global $config;

    echo 'Dossier temporaire des templates : ';
    switch($config['templates']['engine']) {
    case 'smarty':
        echo HTML_OK;
        checkSmarty();
        return true;
        break;
    case 'simpleTemplatesEngine':
        echo HTML_OK;
        checkSimpleTemplatesEngine();
        return true;
        break;
    default:
        echo HTML_NOK;
        return false;
    }
}

function checkSmarty()
{
    global $config;

    echo 'Vérification de la configuration de smarty : ';
    if (is_writable(TEMPLATES_COMPILED_DIR)) {
        echo HTML_OK;
        return true;
    } else {
        // Tente de mettre les bons droits sur templates_c
        if (chmod(TEMPLATES_COMPILED_DIR,TEMPLATES_COMPILED_DIR_MOD) AND is_writable(TEMPLATES_DIR)) {
            echo HTML_OK;
            return true;
        } else {
            echo HTML_NOK;
            return false;
        }
    }
}

function checkSimpleTemplatesEngine()
{
    global $config;

    echo 'Vérification de la configuration de SimpleTemplatesEngine : ';
    echo HTML_OK;
    return true;
}

function checkCots()
{
    global $config;

    echo 'Vérification de la présence des COTS : ';
    //TODO check cots and their version
    
    echo HTML_NOK;
    return false;
}

function downloadCots()
{
    global $config;

    //TODO dowload cots
}

if (checkConfigFile()) {
    checkTemplatesDir();
    if (!checkCots()) {
        downloadCots();
    }
} else {
    createConfigFile();
}

?>
