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

	/*~*~*~*~*~*~*~*~*~*~*/
	/*  1. propriétés    */
	/*~*~*~*~*~*~*~*~*~*~*/

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



	/*~*~*~*~*~*~*~*~*~*~*/
	/*  2. méthodes      */
	/*~*~*~*~*~*~*~*~*~*~*/

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

	/*~*~*~*~*~*~*~*~*~*~*~*~*~*/
	/*  2.1 méthodes privées   */
	/*~*~*~*~*~*~*~*~*~*~*~*~*~*/

	private function instanciateDb() {
		$this->db = null;
		$sgdbName = ucFirst($this->getConfig('sgbd','type'));
		$dbClass = ROOT.'/inc/engine/Db/'.$sgdbName.'.class.php';
		if(file_exists($dbClass)) {
			require_once($dbClass);
			$className = 'Bdf'.$sgdbName.'Db';
			// PHP 5.3
			//$this->db = $className::getInstance();
			//PHP <= 5.2
			$this->db = call_user_func(array($className, 'getInstance'));
		}
		if(null == $this->db) {
			trigger_error('Le sgbd '.$this->getConfig('sgbd','type').' n\'est pas supporté par botte de foin.',E_USER_FATAL);
		}
	}

	private function loadConfiguration() {
		$this->config = parse_ini_file(CONF."config.ini",true);
	}

	public function initialization() {
		if($this->initialized) {
			$this->logger->warn('Core ne peut être initialisé qu\'une seule fois');
		} else {
			$this->loadConfiguration();
			$this->classLoaderInitialization();
			$this->loggerInitialization();
			$this->doctrineInitialization();
			$this->smartyInitialization();
			$this->initialized = true;
		}
	}

	private function loggerInitialization() {
		require_once(ENGINE.'Logger/Logger.class.php');
		$this->logger = namespace\Logging\Logger::getInstance();
		$this->logger->setLevel($this->getConfig("logger","level"));
	}

	private function classLoaderInitialization() {
		require_once(COTS.'doctrine/'.$this->getConfig('doctrine','version').'/Doctrine/Common/IsolatedClassLoader.php');
		$classLoader = new \Doctrine\Common\IsolatedClassLoader('Doctrine');
		$classLoader->setBasePath(COTS.'doctrine/'.$this->getConfig('doctrine','version'));
		$classLoader->register();

		// TODO Bdf Class Loader
	}

	private function doctrineInitialization() {
		$this->doctrineConfig = new \Doctrine\ORM\Configuration();
		$this->doctrineConfig->setProxyDir(INC.$this->getConfig('doctrine','proxy_dir'));
		$this->doctrineConfig->setProxyNamespace($this->getConfig('doctrine','proxy_namespace'));
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
		return $this->config[$section][$var];
	}

	public function getDbConnection() {
		return $this->dbConnection;
	}

	public function getEntityManager() {
		return $this->doctrineEntityManager;
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

	/*~*~*~*~*~*~*~*~*~*~*~*~*~*/
	/*  2.1 méthodes privées   */
	/*~*~*~*~*~*~*~*~*~*~*~*~*~*/

	/**
	 * Nom de la fonction
	 *
	 * <p>Description de la fonction</p>
	 *
	 * @name maClass::maFonction()
	 * @param void
	 * @return void
	 */


	/**
	 * Destructeur
	 *
	 * <p>Description</p>
	 *
	 * @name maClass::__destruct()
	 * @param void
	 * @return void
	 */

	public function __destruct() {

	}

}

?>
