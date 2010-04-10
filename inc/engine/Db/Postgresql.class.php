<?php

class BdfPostgresqlDb implements BdfIDb {

  private $link;
  private static $instance;
  private $lastQuery;
  private $server;
  private $user;
  private $password;

  public static function getInstance() {
    if (!isset(self::$instance)) {
      $c = __CLASS__;
      self::$instance = new $c;
    }

    return self::$instance;
  }

  public function __clone() {
    trigger_error('Le clônage n\'est pas autorisé.', E_USER_ERROR);
  }

  /**
   * affected_rows
   * <p> Get number of affected rows in previous MySQL operation</p>
   * 
   * 
   * @name maClass:affected_rows
   * @param void
   * @return void
   **/

  public function affectedRows() { // Get number of affected rows in previous MySQL operation
    return pg_affected_rows();
  }

  /**
   * change_user
   * <p> Change logged in user of the active connection</p>
   * 
   * 
   * @name maClass:change_user
   * @param void
   * @return void
   **/

  public function changeUser() { // Change logged in user of the active connection
    return pg_change_user();
  }

  /**
   * client_encoding
   * <p> Returns the name of the character set</p>
   * 
   * 
   * @name maClass:client_encoding
   * @param void
   * @return void
   **/

  public function clientEncoding() { // Returns the name of the character set
    return pg_client_encoding();
  }

  /**
   * close
   * <p> Close MySQL connection</p>
   * 
   * 
   * @name maClass:close
   * @param void
   * @return void
   **/

  public function close() { // Close MySQL connection
    return pg_close();
  }

  /**
   * connect
   * <p> Open a connection to a MySQL Server</p>
   * 
   * 
   * @name maClass:connect
   * @param void
   * @return void
   **/

  public function connect($server,$username,$password) { // Open a connection to a MySQL Server
    $this->server = $server;
    $this->username = $username;
    $this->password = $password;
  }

  /**
   * create_db
   * <p> Create a MySQL database</p>
   * 
   * 
   * @name maClass:create_db
   * @param void
   * @return void
   **/

  public function createDb() { // Create a MySQL database
    return pg_create_db();
  }

  /**
   * data_seek
   * <p> Move internal result pointer</p>
   * 
   * 
   * @name maClass:data_seek
   * @param void
   * @return void
   **/

  public function dataSeek() { // Move internal result pointer
    return pg_data_seek();
  }

  /**
   * db_name
   * <p> Get result data</p>
   * 
   * 
   * @name maClass:db_name
   * @param void
   * @return void
   **/

  public function dbName() { // Get result data
    return pg_db_name();
  }

  /**
   * db_query
   * <p> Send a MySQL query</p>
   * 
   * 
   * @name maClass:db_query
   * @param void
   * @return void
   **/

  public function dbQuery() { // Send a MySQL query
    return pg_db_query();
  }

  /**
   * drop_db
   * <p> Drop (delete) a MySQL database</p>
   * 
   * 
   * @name maClass:drop_db
   * @param void
   * @return void
   **/

  public function dropDb() { // Drop (delete) a MySQL database
    return pg_drop_db();
  }

  /**
   * errno
   * <p> Returns the numerical value of the error message from previous MySQL operation</p>
   * 
   * 
   * @name maClass:errno
   * @param void
   * @return void
   **/

  public function errno() { // Returns the numerical value of the error message from previous MySQL operation
    return pg_errno();
  }

  /**
   * error
   * <p> Returns the text of the error message from previous MySQL operation</p>
   * 
   * 
   * @name maClass:error
   * @param void
   * @return void
   **/

  public function error() { // Returns the text of the error message from previous MySQL operation
    return pg_last_error();
  }

  /**
   * escape_string
   * <p> Escapes a string for use in a pg_query</p>
   * 
   * 
   * @name maClass:escape_string
   * @param void
   * @return void
   **/

  public function escapeString() { // Escapes a string for use in a pg_query
    return pg_escape_string();
  }

  /**
   * fetch_array
   * <p> Fetch a result row as an associative array, a numeric array, or both</p>
   * 
   * 
   * @name maClass:fetch_array
   * @param void
   * @return void
   **/

  public function fetchArray($res,$result_type=MYSQL_ASSOC) { // Fetch a result row as an associative array, a numeric array, or both
    if(!is_resource($res) OR get_resource_type($res) !== 'pg result') {
      BdfCore::getInstance()->logger->error('Result argument is not a valid pg result','SQL');
    }

    $result = pg_fetch_array($res,$result_type);

    return $result;
  }

  /**
   * fetch_assoc
   * <p> Fetch a result row as an associative array</p>
   * 
   * 
   * @name maClass:fetch_assoc
   * @param void
   * @return void
   **/

  public function fetchAssoc() { // Fetch a result row as an associative array
    return pg_fetch_assoc();
  }

  /**
   * fetch_field
   * <p> Get column information from a result and return as an object</p>
   * 
   * 
   * @name maClass:fetch_field
   * @param void
   * @return void
   **/

  public function fetchField() { // Get column information from a result and return as an object
    return pg_fetch_field();
  }

  /**
   * fetch_lengths
   * <p> Get the length of each output in a result</p>
   * 
   * 
   * @name maClass:fetch_lengths
   * @param void
   * @return void
   **/

  public function fetchLengths() { // Get the length of each output in a result
    return pg_fetch_lengths();
  }

  /**
   * fetch_object
   * <p> Fetch a result row as an object</p>
   * 
   * 
   * @name maClass:fetch_object
   * @param void
   * @return void
   **/

  public function fetchObject() { // Fetch a result row as an object
    return pg_fetch_object();
  }

  /**
   * fetch_row
   * <p> Get a result row as an enumerated array</p>
   * 
   * 
   * @name maClass:fetch_row
   * @param void
   * @return void
   **/

  public function fetchRow($res) { // Get a result row as an enumerated array
    return pg_fetch_row($res);
  }

  /**
   * field_flags
   * <p> Get the flags associated with the specified field in a result</p>
   * 
   * 
   * @name maClass:field_flags
   * @param void
   * @return void
   **/

  public function fieldFlags() { // Get the flags associated with the specified field in a result
    return pg_field_flags();
  }

  /**
   * field_len
   * <p> Returns the length of the specified field</p>
   * 
   * 
   * @name maClass:field_len
   * @param void
   * @return void
   **/

  public function fieldLen() { // Returns the length of the specified field
    return pg_field_len();
  }

  /**
   * field_name
   * <p> Get the name of the specified field in a result</p>
   * 
   * 
   * @name maClass:field_name
   * @param void
   * @return void
   **/

  public function fieldName() { // Get the name of the specified field in a result
    return pg_field_name();
  }

  /**
   * field_seek
   * <p> Set result pointer to a specified field offset</p>
   * 
   * 
   * @name maClass:field_seek
   * @param void
   * @return void
   **/

  public function fieldSeek() { // Set result pointer to a specified field offset
    return pg_field_seek();
  }

  /**
   * field_table
   * <p> Get name of the table the specified field is in</p>
   * 
   * 
   * @name maClass:field_table
   * @param void
   * @return void
   **/

  public function fieldTable() { // Get name of the table the specified field is in
    return pg_field_table();
  }

  /**
   * field_type
   * <p> Get the type of the specified field in a result</p>
   * 
   * 
   * @name maClass:field_type
   * @param void
   * @return void
   **/

  public function fieldType() { // Get the type of the specified field in a result
    return pg_field_type();
  }

  /**
   * free_result
   * <p> Free result memory</p>
   * 
   * 
   * @name maClass:free_result
   * @param void
   * @return void
   **/

  public function freeResult() { // Free result memory
    return pg_free_result();
  }

  /**
   * get_client_info
   * <p> Get MySQL client info</p>
   * 
   * 
   * @name maClass:get_client_info
   * @param void
   * @return void
   **/

  public function getClient_info() { // Get MySQL client info
    return pg_get_client_info();
  }

  /**
   * get_host_info
   * <p> Get MySQL host info</p>
   * 
   * 
   * @name maClass:get_host_info
   * @param void
   * @return void
   **/

  public function getHost_info() { // Get MySQL host info
    return pg_get_host_info();
  }

  /**
   * get_proto_info
   * <p> Get MySQL protocol info</p>
   * 
   * 
   * @name maClass:get_proto_info
   * @param void
   * @return void
   **/

  public function getProto_info() { // Get MySQL protocol info
    return pg_get_proto_info();
  }

  /**
   * get_server_info
   * <p> Get MySQL server info</p>
   * 
   * 
   * @name maClass:get_server_info
   * @param void
   * @return void
   **/

  public function getServer_info() { // Get MySQL server info
    return pg_get_server_info();
  }

  /**
   * info
   * <p> Get information about the most recent query</p>
   * 
   * 
   * @name maClass:info
   * @param void
   * @return void
   **/

  public function info() { // Get information about the most recent query
    return pg_info();
  }

  /**
   * insert_id
   * <p> Get the ID generated from the previous INSERT operation</p>
   * 
   * 
   * @name maClass:insert_id
   * @param void
   * @return void
   **/

  public function insertId() { // Get the ID generated from the previous INSERT operation
    return pg_insert_id();
  }

  /**
   * list_dbs
   * <p> List databases available on a MySQL server</p>
   * 
   * 
   * @name maClass:list_dbs
   * @param void
   * @return void
   **/

  public function listDbs() { // List databases available on a MySQL server
    return pg_list_dbs();
  }

  /**
   * list_fields
   * <p> List MySQL table fields</p>
   * 
   * 
   * @name maClass:list_fields
   * @param void
   * @return void
   **/

  public function listFields() { // List MySQL table fields
    return pg_list_fields();
  }

  /**
   * list_processes
   * <p> List MySQL processes</p>
   * 
   * 
   * @name maClass:list_processes
   * @param void
   * @return void
   **/

  public function listProcesses() { // List MySQL processes
    return pg_list_processes();
  }

  /**
   * list_tables
   * <p> List tables in a MySQL database</p>
   * 
   * 
   * @name maClass:list_tables
   * @param void
   * @return void
   **/

  public function listTables() { // List tables in a MySQL database
    return pg_list_tables();
  }

  /**
   * num_fields
   * <p> Get number of fields in result</p>
   * 
   * 
   * @name maClass:num_fields
   * @param void
   * @return void
   **/

  public function numFields() { // Get number of fields in result
    return pg_num_fields();
  }

  /**
   * num_rows
   * <p> Get number of rows in result</p>
   * 
   * 
   * @name maClass:num_rows
   * @param void
   * @return void
   **/

  public function numRows($result) { // Get number of rows in result
    if(!is_resource($result) OR get_resource_type($result) != 'pg result') {
      BdfCore::getInstance()->logger->error('Result argument is not a valid pg result','SQL');
      return false;
    }
    return pg_num_rows($result);
  }

  /**
   * pconnect
   * <p> Open a persistent connection to a MySQL server</p>
   * 
   * 
   * @name maClass:pconnect
   * @param void
   * @return void
   **/

  public function pconnect() { // Open a persistent connection to a MySQL server
    return pg_pconnect();
  }

  /**
   * ping
   * <p> Ping a server connection or reconnect if there is no connection</p>
   * 
   * 
   * @name maClass:ping
   * @param void
   * @return void
   **/

  public function ping() { // Ping a server connection or reconnect if there is no connection
    return pg_ping();
  }

  /**
   * query
   * <p> Send a MySQL query</p>
   * 
   * 
   * @name maClass:query
   * @param void
   * @return void
   **/

  public function query($query) { // Send a MySQL query
    $this->lastQuery = $query;

    $result = pg_query($query);
    if($result === false) {
      BdfCore::getInstance()->logger->error($query."\n".$this->error(),'SQL');
    }

    return $result;
  }

  /**
   * real_escape_string
   * <p> Escapes special characters in a string for use in a SQL statement</p>
   * 
   * 
   * @name maClass:real_escape_string
   * @param void
   * @return void
   **/

  public function realEscapeString($string) { // Escapes special characters in a string for use in a SQL statement
    return pg_escape_string($string);
  }

  /**
   * result
   * <p> Get result data</p>
   * 
   * 
   * @name maClass:result
   * @param void
   * @return void
   **/

  public function result($result,$row,$field=0) { // Get result data
    $temp = pg_result($result,$row,$field);
    if($temp === false)
      BdfCore::getInstance()->logger->error($this->lastQuery,"SQL");
    return $temp;
  }

  /**
   * select_db
   * <p> Select a MySQL database</p>
   * 
   * 
   * @name maClass:select_db
   * @param void
   * @return void
   **/

  public function selectDb($database_name) { // Select a MySQL database
    $this->link = pg_connect("host=".$this->server." user=".$this->username." dbname=".$database_name." password=".$this->password) or BdfCore::getInstance()->logger->fatal("Connexion impossible à la base de donnée","Connection à la base de donnée");
    return $this->link;
  }

  /**
   * set_charset
   * <p> Sets the client character set</p>
   * 
   * 
   * @name maClass:set_charset
   * @param void
   * @return void
   **/

  public function setCharset() { // Sets the client character set
    return pg_set_charset();
  }

  /**
   * stat
   * <p> Get current system status</p>
   * 
   * 
   * @name maClass:stat
   * @param void
   * @return void
   **/

  public function stat() { // Get current system status
    return pg_stat();
  }

  /**
   * tablename
   * <p> Get table name of field</p>
   * 
   * 
   * @name maClass:tablename
   * @param void
   * @return void
   **/

  public function tablename() { // Get table name of field
    return pg_tablename();
  }

  /**
   * thread_id
   * <p> Return the current thread ID</p>
   * 
   * 
   * @name maClass:thread_id
   * @param void
   * @return void
   **/

  public function threadId() { // Return the current thread ID
    return pg_thread_id();
  }

  /**
   * unbuffered_query
   * <p> Send an SQL query to MySQL, without fetching and buffering the result rows</p>
   * 
   * 
   * @name maClass:unbuffered_query
   * @param void
   * @return void
   **/

  public function unbufferedQuery() { // Send an SQL query to MySQL, without fetching and buffering the result rows
    return pg_unbuffered_query();
  }
}
?>
