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
 * Core
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://dev.paulfariello.fr/utt/lo07/
 */

class Core
{

    /**
     * @var type
     * @desc description
     */
    private static $_instance = null;
    private $_secu = 'none';
    private $_listeDroit = array();
    private $_userId = 0;

    /**
     * @var type
     * @desc description
     */

    private $_config = array();
    private $_initialized = false;
    private $_session = null;

    private $_doctrineConfig = null;
    private $_dbConnection = null;
    private $_doctrineEntityManager = null;

    private $_templatesEngine = null;



    /**
     * Constructeur
     *
     * @return Core
     */

    private function __construct()
    {

    }

    /**
     * Accesseur à l'instance de Core
     *
     * @return Core
     */
    public static function getInstance()
    {
        if (self::$instance === null) {
            $c = __CLASS__;
            self::$instance = new $c;
        }

        return self::$instance;
    }

    /**
     * Redefinition de __clone pour interfire le clonnage de l'instance de Core
     *
     * @return void
     */
    public function __clone()
    {
        trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
    }

    /**
     * Chargement de la configuration via les fichiers *.ini
     *
     * @return void
     */
    private function _loadConfiguration()
    {
        if (file_exists(CONF."config.ini")) {
            $this->_config = parse_ini_file(CONF."config.ini", true);
        } else {
            trigger_error('Le fichier de configuration n\'est pas présent. Utiliser le script install/index.php pour en créer un nouveau.', E_USER_ERROR);
        }
    }

    /**
     * Initialisation de la conf et des outils
     *
     * @return void
     */
    public function initialization()
    {
        if ($this->_initialized) {
            $this->logger->warn('Core ne peut être initialisé qu\'une seule fois');
        } else {
            $this->_loadConfiguration();
            $this->_classLoaderInitialization();
            $this->_loggerInitialization();
            $this->_doctrineInitialization();
            $this->_sessionInitialization();
            $this->_templatesEngineInitialization();
            $this->_initialized = true;
        }
    }

    /**
     * Initilisation du Logger
     *
     * @return void
     */
    private function _loggerInitialization()
    {
        $this->logger = Logger::getInstance();
        $this->logger->setLevel($this->getConfig("logger", "level"));
    }

    /**
     * Initialisation des Class Loader
     *
     * @return void
     */
    private function _classLoaderInitialization()
    {
        include_once COTS.'doctrine/'.$this->getConfig('doctrine', 'version').'/Doctrine/Common/ClassLoader.php';

        // Register Bdf Class Loader
        $classLoader = new \Doctrine\Common\ClassLoader('Bdf');
        $classLoader->setIncludePath(ENGINE);
        $classLoader->setFileExtension('.class.php');
        $classLoader->register();

        // Register Doctrine Class Loader
        $classLoader = new \Doctrine\Common\ClassLoader('Doctrine');
        $classLoader->setIncludePath(COTS.'doctrine/'.$this->getConfig('doctrine', 'version'));
        $classLoader->register();

        // Register Bdf Class Loader
        $classLoader = new \Doctrine\Common\ClassLoader($this->getConfig('site', 'namespace'));
        $classLoader->setIncludePath(INC."class/");
        $classLoader->setFileExtension('.class.php');
        $classLoader->register();
    }

    /**
     * Configuration et initialisation de Doctrine
     *
     * @return void
     */
    private function _doctrineInitialization()
    {
        $this->_doctrineConfig = new \Doctrine\ORM\Configuration();
        $this->_doctrineConfig->setProxyDir(INC.$this->getConfig('doctrine', 'proxy_dir'));
        $this->_doctrineConfig->setProxyNamespace($this->getConfig('doctrine', 'proxy_namespace'));
        // Deprecated for production environment
        $this->_doctrineConfig->setAutoGenerateProxyClasses(true);
        $driver = new \Doctrine\ORM\Mapping\Driver\XmlDriver(array(INC.$this->getConfig('doctrine', 'mapping_dir')));
        $this->_doctrineConfig->setMetadataDriverImpl($driver);
        $this->_doctrineConfig->setSqlLogger($this->logger);

        if ($this->getConfig('doctrine', 'cache') == 'apc') {
            $cache = new \Doctrine\Common\Cache\ApcCache();
            $this->_doctrineConfig->setMetadataCacheImpl($cache);
            $this->_doctrineConfig->setQueryCacheImpl($cache);
        }

        $connectionParams = array(
                'dbname' => $this->getConfig('sgbd', 'database_name'),
                'user' => $this->getConfig('sgbd', 'user_name'),
                'password' => $this->getConfig('sgbd', 'password'),
                'host' => $this->getConfig('sgbd', 'host'),
                'driver' => $this->getConfig('sgbd', 'driver')
        );
        $this->_dbConnection = \Doctrine\DBAL\DriverManager::getConnection($connectionParams);
        $this->_doctrineEntityManager = \Doctrine\ORM\EntityManager::create($this->_dbConnection, $this->_doctrineConfig);
    }

    /**
     * Configuration et initialisation du moteur de templates
     *
     * Cette fonction ce contente d'appeler le bon initialiseur
     *
     * @return void
     */
    private function _templatesEngineInitialization()
    {
        switch($this->getConfig('templates', 'engine')) {
        case "smarty":
            $this->_smartyInitialization();
            break;
        case "simpleTemplatesEngine":
            $this->_simpleTemplatesEngineInitialization();
            break;
        default:
            trigger_error('Mauvaise configuration du moteur de templates');
        }
    }

    /**
     * Initialisation du moteur de templates simpleTemplatesEngine
     *
     * @return void
     */
    private function _simpleTemplatesEngineInitialization()
    {
        $this->_templatesEngine = new SimpleTemplatesEngine();
        $this->_templatesEngine->initialization();
        $this->_templatesEngine->setSkin($this->getConfig("site", "skin"));
    }

    /**
     * Initialisation du moteur de templates Smarty
     *
     * @return void
     */
    private function _smartyInitialization()
    {
        $this->_templatesEngine = new SmartyAdapter();
        $this->_templatesEngine->initialization();
        $this->_templatesEngine->setSkin($this->getConfig("site", "skin"));
    }

    /**
     * Initialisation des sessions
     *
     * @return void
     */
    private function _sessionInitialization()
    {
        $this->_session = Session::getInstance();
    }


    /**
     * Accesseur à l'instance du moteur de templates
     *
     * @return Smarty instance or SimpleTemplatesEngine instance
     */
    public function getTemplatesEngine()
    {
        return $this->_templatesEngine;
    }


    /**
     * Accesseur à un élément de la configuration
     *
     * @param string $section La section de configuration
     * @param string $var     La variable de configuration
     * 
     * @return string
     */
    public function getConfig($section,$var)
    {
        if (isset($this->_config[$section][$var])) {
            return $this->_config[$section][$var];
        }
        return null;
    }

    /**
     * Accesseur au Driver de connection à la base de donnée de Doctrine
     *
     * @return \Doctrine\DBAL\DriverManager
     */
    public function getDbConnection()
    {
        return $this->_dbConnection;
    }

    /**
     * Accesseur à l'Entity Manager de Doctrine
     *
     * @return \Doctrine\ORM\EntityManager
     */
    public function getEntityManager()
    {
        return $this->_doctrineEntityManager;
    }

}

?>
