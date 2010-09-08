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
define('HTML_UNSUPPORTED_VERSION', '<span style="color:orange">Version non supportée</span><br />');
define('HTML_COTS_NOT_FOUND', '<span style="color:orange">Fichiers introuvables</span><br />');

define('ROOT', dirname(__FILE__).'/../');
define('CONFIG_FILE', ROOT.'inc/conf/config.ini');
define('COTS_FILE', ROOT.'install/cots.xml');
define('COTS_DIR', ROOT.'inc/cots/');
define('TEMPLATES_COMPILED_DIR', ROOT.'templates_c/');
define('TEMPLATES_COMPILED_DIR_MOD', 0770);

$config = array();
$cots = new SimpleXMLElement(COTS_FILE, null, true);

if (isset($_POST['create-config-file'])) {
    if (file_exists(CONFIG_FILE)) {
        rename(CONFIG_FILE,CONFIG_FILE.'.'.time().'.old');
    }
    $f = @fopen(CONFIG_FILE,'w+');
    if ($f === false) {
        echo 'Vérifier les permissions du dossier contenant '.CONFIG_FILE;
        die();
    }
    define('EOL',"\n");
    fwrite($f,';Configuration file automaticaly created by BotteDeFoin on '.date('r').EOL);

    fwrite($f,'[logger]'.EOL);
    fwrite($f,';See possible values in Bdf\Logger class'.EOL);
    fwrite($f,'level = "'.$_POST['logger-level'].'"'.EOL);
    fwrite($f,EOL);

    fwrite($f,'[sgbd]'.EOL);
    fwrite($f,';See supported drivers by DBAL from Doctrine project'.EOL);
    fwrite($f,'driver = "'.$_POST['sgbd-driver'].'"'.EOL);
    fwrite($f,'host = "'.$_POST['sgbd-host'].'"'.EOL);
    fwrite($f,'user = "'.$_POST['sgbd-user'].'"'.EOL);
    fwrite($f,'password = "'.$_POST['sgbd-password'].'"'.EOL);
    fwrite($f,'database_name= "'.$_POST['sgbd-database-name'].'"'.EOL);
    fwrite($f,';Hash method is used for encrypted password stored in database'.EOL);
    fwrite($f,';See possible values with hash_algos() php function'.EOL);
    fwrite($f,'hash = "'.$_POST['sgbd-hash'].'"'.EOL);
    fwrite($f,EOL);

    fwrite($f,'[doctrine]'.EOL);
    fwrite($f,'release = "'.$_POST['doctrine-release'].'"'.EOL);
    fwrite($f,'proxy_dir = "'.$_POST['doctrine-proxy-dir'].'"'.EOL);
    fwrite($f,";Namespace should not start with \\".EOL);
    fwrite($f,'proxy_namespace = "'.$_POST['doctrine-proxy-namespace'].'"'.EOL);
    fwrite($f,'mapping_dir = "'.$_POST['doctrine-mapping-dir'].'"'.EOL);
    fwrite($f,';See cache engine supported by your version of Doctrine'.EOL);
    if ($_POST['doctrine-cache'] != 'none') {
        fwrite($f,'cache = "'.$_POST['doctrine-cache'].'"'.EOL);
    } else {
        fwrite($f,';cache = ""');
    }
    fwrite($f,EOL);

    fwrite($f,'[templates]'.EOL);
    list($templatesEngine,$templatesEngineRelease) = explode ('-', $_POST['templates-engine'], 2);
    fwrite($f,'engine = "'.$templatesEngine.'"'.EOL);
    fwrite($f,'release = "'.$templatesEngineRelease.'"'.EOL);
    fwrite($f,'dir = "'.$_POST['templates-dir'].'"'.EOL);
    fwrite($f,";Line can be commented if your templates engine doesn't need special directory for compiled templates files".EOL);
    fwrite($f,'compiled_dir = "'.$_POST['templates-compiled-dir'].'"'.EOL);
    fwrite($f,EOL);

    fwrite($f,'[site]'.EOL);
    fwrite($f,";Root url of your site. Used when you create link".EOL);
    fwrite($f,'url = "'.$_POST['site-url'].'"'.EOL);
    fwrite($f,";You should store images that are used with css here.".EOL.";That means no user uploaded images, and more generaly no images that represents site contents. My 2 cents...".EOL);
    fwrite($f,'image_dir = "'.$_POST['site-image-dir'].'"'.EOL);
    fwrite($f,'style_dir = "'.$_POST['site-style-dir'].'"'.EOL);
    fwrite($f,'javascript_dir = "'.$_POST['site-javascript-dir'].'"'.EOL);
    fwrite($f,'skin = "'.$_POST['site-skin'].'"'.EOL);
    fwrite($f,";Namespace should not start with \\".EOL);
    fwrite($f,'namespace = "'.$_POST['site-namespace'].'"'.EOL);

    fclose($f);
    header('location: index.php');
}

?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    <title>BotteDeFoin - Installation</title>
</head>
<body>
<form action="config.php" method="post">
    <fieldset>
        <legend>Logger</legend>
        <label for="logger-level">Level</label>
        <select name="logger-level">
            <option value="DEBUG">DEBUG</option>
            <option value="INFO">INFO</option>
            <option value="WARN">WARN</option>
            <option value="ERROR">ERROR</option>
            <option value="FATAL">FATAL</option>
        </select>
    </fieldset>
    <fieldset>
        <legend>SGBD</legend>
        <label for="sgbd-driver">Driver</label>
        <select name="sgbd-driver">
            <option value="pdo_mysql">pdo_mysql</option>
            <option value="pdo_sqlite">pdo_sqlite</option>
            <option value="pdo_pgsql">pdo_pgsql</option>
            <option value="pdo_oci">pdo_oci</option>
            <option value="pdo_mssql">pdo_mssql</option>
            <option value="oci8">oci8</option>
        </select><br />
        <label for="sgbd-host">Host</label><input name="sgbd-host" type="text" value="localhost" /><br />
        <label for="sgbd-user">User</label><input name="sgbd-user" type="text" value="" /><br />
        <label for="sgbd-password">Password</label><input name="sgbd-password" type="text" value="" /><br />
        <label for="sgbd-database-name">Database</label><input name="sgbd-database-name" type="text" value="" /><br />
        <label for="sgbd-hash">Hash</label>
        <select name="sgbd-hash">
            <?php foreach(hash_algos() as $hash): ?>
            <option value="<?php echo $hash; ?>"><?php echo $hash; ?></option>
            <?php endforeach; ?>
        </select>
    </fieldset>
    <fieldset>
        <legend>Doctrine</legend>
        <label for="doctrine-release">Version de doctrine</label>
        <select name="doctrine-release">
            <?php foreach($cots->doctrine->children() as $release): ?>
            <option value="<?php echo (string)$release->attributes()->name; ?>"><?php echo (string)$release->attributes()->name; ?></option>
            <?php endforeach; ?>
        </select><br />
        <label for="doctrine-proxy-dir">Proxy directory</label><input name="doctrine-proxy-dir" type="text" value="inc/proxy/" /><br />
        <label for="doctrine-proxy-namespace">Proxy namespace (do not start with \)</label><input name="doctrine-proxy-namespace" type="text" value="Bdf\Model\Proxy" /><br />
        <label for="doctrine-mapping-dir">Mapping directory</label><input name="doctrine-mapping-dir" type="text" value="inc/mapping/" /><br />
        <label for="doctrine-cache">Cache</label>
        <select name="doctrine-cache">
            <option value="none">Aucun</option>
            <option value="apc">APC</option>
        </select>
    </fieldset>
    <fieldset>
        <legend>Templates</legend>
        <label for="templates-engine">Engine</label>
        <select name="templates-engine">
            <?php foreach($cots->templates->children() as $engine): ?>
            <optgroup label="<?php echo (string)$engine->attributes()->name; ?>">
                <?php foreach($engine->children() as $release): ?>
                <option value="<?php echo (string)$engine->attributes()->name.'-'.(string)$release->attributes()->name; ?>"><?php echo (string)$engine->attributes()->name.' '.(string)$release->attributes()->name; ?></option>
                <?php endforeach; ?>
            </optgroup>
            <?php endforeach; ?>
        </select><br />
        <label for="templates-dir">Templates directory</label><input name="templates-dir" type="text" value="templates/" /><br />
        <label for="templates-compiled-dir">Templates compiled directory</label><input name="templates-compiled-dir" type="text" value="templates_c/" /><br />
    </fieldset>
    <fieldset>
        <legend>Site</legend>
        <label for="site-url">Url</label><input name="site-url" type="text" value="http://" /><br />
        <label for="site-image-dir">Image directory</label><input name="site-image-dir" type="text" value="px/" /><br />
        <label for="site-style-dir">Css directory</label><input name="site-style-dir" type="text" value="style/" /><br />
        <label for="site-javascript-dir">Js directory</label><input name="site-javascript-dir" type="text" value="js/" /><br />
        <label for="site-skin">Skin</label><input name="site-skin" type="text" value="default" /><br />
        <label for="site-namespace">Namespace</label><input name="site-namespace" type="text" value="" /><br />
    </fieldset>
    <fieldset>
        <legend>Javascript</legend>
        <label for="javascript-framework">Framework</label>
        <select name="javascript-framework">
            <?php foreach($cots->javascript->children() as $framework): ?>
            <optgroup label="<?php echo (string)$framework->attributes()->name; ?>">
                <?php foreach($framework->children() as $release): ?>
                <option value="<?php echo (string)$framework->attributes()->name.'-'.$release->attributes()->name; ?>"><?php echo (string)$framework->attributes()->name.' '.(string)$release->attributes()->name; ?></option>
                <?php endforeach; ?>
            </optgroup>
            <?php endforeach; ?>
        </select><br />
        <label for="javascript-framework-dir">Framework directory</label><input name="javascript-framework-dir" type="text" value="js/lib/" /><br />
    </fieldset>
    <fieldset>
        <legend>Options</legend>
        <input type="checkbox" name="create-directories" value="create-directories" checked="checked" /><label for="create-directories">Créer les dossiers requis</label>
    </fieldset>
    <input type="submit" name="create-config-file" value="Créer le fichier de configuration" />
</form>
</body>
</html>
