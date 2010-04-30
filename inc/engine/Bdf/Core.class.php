<?php

/**
 * BdfCore
 *
 *
 * <p>Description de la class</p>
 *
 * @name nom
 * @author Needle <paul.fariello@gmail.com>
 * @link
 * @copyright Paul Fariello 2007
 * @version 1.0.0
 * @package Nom du package
 */

namespace Bdf;

class Core {

	/**
	 * @var type
	 * @desc description
	 */
	private static $instance = null;
	private $secu = 'none';
	private $listeDroit = array();
	private $userId = 0;

	/**
	 * @var type
	 * @desc description
	 */

	private $config = array();
	private $initialized = false;
	private $utils = null;
	private $session = null;

	private $doctrineConfig = null;
	private $dbConnection = null;
	private $doctrineEntityManager = null;

  private $templatesEngine = null;



	/**
	 * Constructeur
	 *
	 * <p>description</p>
	 *
	 * @name maClass::__construct()
	 * @return void
	 */

	private function __construct() {

	}


	public static function getInstance() {
		if (self::$instance === null) {
			$c = __CLASS__;
			self::$instance = new $c;
		}

		return self::$instance;
	}

	public function __clone() {
		trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
	}

	private function loadConfiguration() {
    if(file_exists(CONF."config.ini")) {
      $this->config = parse_ini_file(CONF."config.ini",true);
    } else {
			trigger_error('Le fichier de configuration n\'est pas présent. Utiliser le script install/index.php pour en créer un nouveau.',E_USER_ERROR);
    }
	}

	public function initialization() {
		if($this->initialized) {
			$this->logger->warn('Core ne peut être initialisé qu\'une seule fois');
		} else {
			$this->loadConfiguration();
			$this->classLoaderInitialization();
			$this->loggerInitialization();
			$this->utilsInitialization();
			$this->doctrineInitialization();
      $this->sessionInitialization();
			$this->smartyInitialization();
			$this->initialized = true;
		}
	}

	private function loggerInitialization() {
		$this->logger = \Bdf\Logger::getInstance();
		$this->logger->setLevel($this->getConfig("logger","level"));
	}

  private function utilsInitialization() {
    $this->utils = \Bdf\Utils::getInstance();
  }

	private function classLoaderInitialization() {
		require_once(COTS.'doctrine/'.$this->getConfig('doctrine','version').'/Doctrine/Common/IsolatedClassLoader.php');

    // Register Bdf Class Loader
		$classLoader = new \Doctrine\Common\IsolatedClassLoader('Bdf');
		$classLoader->setBasePath(ENGINE);
		$classLoader->setFileExtension('.class.php');
		$classLoader->register();

    // Register Doctrine Class Loader
		$classLoader = new \Doctrine\Common\IsolatedClassLoader('Doctrine');
		$classLoader->setBasePath(COTS.'doctrine/'.$this->getConfig('doctrine','version'));
		$classLoader->register();
    
    // Register Bdf Class Loader
		$classLoader = new \Doctrine\Common\IsolatedClassLoader($this->getConfig('site','namespace'));
		$classLoader->setBasePath(INC."class/");
		$classLoader->setFileExtension('.class.php');
		$classLoader->register();
	}

	private function doctrineInitialization() {
		$this->doctrineConfig = new \Doctrine\ORM\Configuration();
		$this->doctrineConfig->setProxyDir(INC.$this->getConfig('doctrine','proxy_dir'));
		$this->doctrineConfig->setProxyNamespace($this->getConfig('doctrine','proxy_namespace'));
    // Deprecated for production environment
    $this->doctrineConfig->setAutoGenerateProxyClasses(true);
		$driver = new \Doctrine\ORM\Mapping\Driver\XmlDriver(array(INC.$this->getConfig('doctrine','mapping_dir')));
		$this->doctrineConfig->setMetadataDriverImpl($driver);
		$this->doctrineConfig->setSqlLogger($this->logger);

		if($this->getConfig('doctrine','cache') == 'apc') {
			$cache = new \Doctrine\Common\Cache\ApcCache();
			$this->doctrineConfig->setMetadataCacheImpl($cache);
			$this->doctrineConfig->setQueryCacheImpl($cache);
		}

		$connectionParams = array(
      'dbname' => $this->getConfig('sgbd','database_name'),
      'user' => $this->getConfig('sgbd','user_name'),
      'password' => $this->getConfig('sgbd','password'),
      'host' => $this->getConfig('sgbd','host'),
      'driver' => $this->getConfig('sgbd','driver')
		);
		$this->dbConnection = \Doctrine\DBAL\DriverManager::getConnection($connectionParams);
		$this->doctrineEntityManager = \Doctrine\ORM\EntityManager::create($this->dbConnection, $this->doctrineConfig);
	}

	private function smartyInitialization() {
    require_once(COTS."smarty/2.6.26-STABLE/Smarty.class.php");
    $this->templatesEngine = new \Smarty();
    $this->templatesEngine->template_dir = ROOT."templates/".$this->session->getUser()->getSkin().'/';
    $this->templatesEngine->compile_dir  = ROOT."templates_c/";
    if($this->getConfig('logger','level') == 'Bdf::DEBUG') {
      $this->templatesEngine->debugging = true;
    } else {
      $this->templatesEngine->debugging = false;
    }

    $this->templatesEngine->assign("bdfUtils",$this->utils);

	}

  private function sessionInitialization() {
    $this->session = \Bdf\Session::getInstance();
  }


  public function getTemplatesEngine() {
    return $this->templatesEngine;
  }


  public function getClientClass($className) {
    return $this->getConfig('site','namespace')."\\".$className;
  }

	/**
	 * Nom de la fonction
	 *
	 * <p>Description de la fonction</p>
	 *
	 * @name maClass::maFonction()
	 * @param void
	 * @return void
	 */


	public function getConfig($section,$var) {
    if(isset($this->config[$section][$var])) {
      return $this->config[$section][$var];
    }
		return null;
	}

	public function getDbConnection() {
		return $this->dbConnection;
	}

	public function getEntityManager() {
		return $this->doctrineEntityManager;
	}

}

?>
