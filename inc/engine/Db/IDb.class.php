<?php

interface BdfIDb {

  /**
   * affected_rows
   * <p> Get number of affected rows in previous MySQL operation</p>
   * 
   * 
   * @name maClass:affected_rows
   * @param void
   * @return void
   **/

  public function affectedRows();

  /**
   * change_user
   * <p> Change logged in user of the active connection</p>
   * 
   * 
   * @name maClass:change_user
   * @param void
   * @return void
   **/

  public function changeUser();

  /**
   * client_encoding
   * <p> Returns the name of the character set</p>
   * 
   * 
   * @name maClass:client_encoding
   * @param void
   * @return void
   **/

  public function clientEncoding();

  /**
   * close
   * <p> Close MySQL connection</p>
   * 
   * 
   * @name maClass:close
   * @param void
   * @return void
   **/

  public function close();

  /**
   * connect
   * <p> Open a connection to a MySQL Server</p>
   * 
   * 
   * @name maClass:connect
   * @param void
   * @return void
   **/

  public function connect($server,$username,$password);

  /**
   * create_db
   * <p> Create a MySQL database</p>
   * 
   * 
   * @name maClass:create_db
   * @param void
   * @return void
   **/

  public function createDb();

  /**
   * data_seek
   * <p> Move internal result pointer</p>
   * 
   * 
   * @name maClass:data_seek
   * @param void
   * @return void
   **/

  public function dataSeek();

  /**
   * db_name
   * <p> Get result data</p>
   * 
   * 
   * @name maClass:db_name
   * @param void
   * @return void
   **/

  public function dbName();

  /**
   * db_query
   * <p> Send a MySQL query</p>
   * 
   * 
   * @name maClass:db_query
   * @param void
   * @return void
   **/

  public function dbQuery();

  /**
   * drop_db
   * <p> Drop (delete) a MySQL database</p>
   * 
   * 
   * @name maClass:drop_db
   * @param void
   * @return void
   **/

  public function dropDb();

  /**
   * errno
   * <p> Returns the numerical value of the error message from previous MySQL operation</p>
   * 
   * 
   * @name maClass:errno
   * @param void
   * @return void
   **/

  public function errno();

  /**
   * error
   * <p> Returns the text of the error message from previous MySQL operation</p>
   * 
   * 
   * @name maClass:error
   * @param void
   * @return void
   **/

  public function error();

  /**
   * escape_string
   * <p> Escapes a string for use in a mysql_query</p>
   * 
   * 
   * @name maClass:escape_string
   * @param void
   * @return void
   **/

  public function escapeString();

  /**
   * fetch_array
   * <p> Fetch a result row as an associative array, a numeric array, or both</p>
   * 
   * 
   * @name maClass:fetch_array
   * @param void
   * @return void
   **/

  public function fetchArray($res,$result_type=MYSQL_ASSOC);

  /**
   * fetch_assoc
   * <p> Fetch a result row as an associative array</p>
   * 
   * 
   * @name maClass:fetch_assoc
   * @param void
   * @return void
   **/

  public function fetchAssoc();

  /**
   * fetch_field
   * <p> Get column information from a result and return as an object</p>
   * 
   * 
   * @name maClass:fetch_field
   * @param void
   * @return void
   **/

  public function fetchField();

  /**
   * fetch_lengths
   * <p> Get the length of each output in a result</p>
   * 
   * 
   * @name maClass:fetch_lengths
   * @param void
   * @return void
   **/

  public function fetchLengths();

  /**
   * fetch_object
   * <p> Fetch a result row as an object</p>
   * 
   * 
   * @name maClass:fetch_object
   * @param void
   * @return void
   **/

  public function fetchObject();

  /**
   * fetch_row
   * <p> Get a result row as an enumerated array</p>
   * 
   * 
   * @name maClass:fetch_row
   * @param void
   * @return void
   **/

  public function fetchRow($res);

  /**
   * field_flags
   * <p> Get the flags associated with the specified field in a result</p>
   * 
   * 
   * @name maClass:field_flags
   * @param void
   * @return void
   **/

  public function fieldFlags();

  /**
   * field_len
   * <p> Returns the length of the specified field</p>
   * 
   * 
   * @name maClass:field_len
   * @param void
   * @return void
   **/

  public function fieldLen();

  /**
   * field_name
   * <p> Get the name of the specified field in a result</p>
   * 
   * 
   * @name maClass:field_name
   * @param void
   * @return void
   **/

  public function fieldName();

  /**
   * field_seek
   * <p> Set result pointer to a specified field offset</p>
   * 
   * 
   * @name maClass:field_seek
   * @param void
   * @return void
   **/

  public function fieldSeek();

  /**
   * field_table
   * <p> Get name of the table the specified field is in</p>
   * 
   * 
   * @name maClass:field_table
   * @param void
   * @return void
   **/

  public function fieldTable();

  /**
   * field_type
   * <p> Get the type of the specified field in a result</p>
   * 
   * 
   * @name maClass:field_type
   * @param void
   * @return void
   **/

  public function fieldType();

  /**
   * free_result
   * <p> Free result memory</p>
   * 
   * 
   * @name maClass:free_result
   * @param void
   * @return void
   **/

  public function freeResult();

  /**
   * get_client_info
   * <p> Get MySQL client info</p>
   * 
   * 
   * @name maClass:get_client_info
   * @param void
   * @return void
   **/

  public function getClient_info();

  /**
   * get_host_info
   * <p> Get MySQL host info</p>
   * 
   * 
   * @name maClass:get_host_info
   * @param void
   * @return void
   **/

  public function getHost_info();

  /**
   * get_proto_info
   * <p> Get MySQL protocol info</p>
   * 
   * 
   * @name maClass:get_proto_info
   * @param void
   * @return void
   **/

  public function getProto_info();

  /**
   * get_server_info
   * <p> Get MySQL server info</p>
   * 
   * 
   * @name maClass:get_server_info
   * @param void
   * @return void
   **/

  public function getServer_info();

  /**
   * info
   * <p> Get information about the most recent query</p>
   * 
   * 
   * @name maClass:info
   * @param void
   * @return void
   **/

  public function info();

  /**
   * insert_id
   * <p> Get the ID generated from the previous INSERT operation</p>
   * 
   * 
   * @name maClass:insert_id
   * @param void
   * @return void
   **/

  public function insertId();

  /**
   * list_dbs
   * <p> List databases available on a MySQL server</p>
   * 
   * 
   * @name maClass:list_dbs
   * @param void
   * @return void
   **/

  public function listDbs();

  /**
   * list_fields
   * <p> List MySQL table fields</p>
   * 
   * 
   * @name maClass:list_fields
   * @param void
   * @return void
   **/

  public function listFields();

  /**
   * list_processes
   * <p> List MySQL processes</p>
   * 
   * 
   * @name maClass:list_processes
   * @param void
   * @return void
   **/

  public function listProcesses();

  /**
   * list_tables
   * <p> List tables in a MySQL database</p>
   * 
   * 
   * @name maClass:list_tables
   * @param void
   * @return void
   **/

  public function listTables();

  /**
   * num_fields
   * <p> Get number of fields in result</p>
   * 
   * 
   * @name maClass:num_fields
   * @param void
   * @return void
   **/

  public function numFields();

  /**
   * num_rows
   * <p> Get number of rows in result</p>
   * 
   * 
   * @name maClass:num_rows
   * @param void
   * @return void
   **/

  public function numRows($result);

  /**
   * pconnect
   * <p> Open a persistent connection to a MySQL server</p>
   * 
   * 
   * @name maClass:pconnect
   * @param void
   * @return void
   **/

  public function pconnect();

  /**
   * ping
   * <p> Ping a server connection or reconnect if there is no connection</p>
   * 
   * 
   * @name maClass:ping
   * @param void
   * @return void
   **/

  public function ping();

  /**
   * query
   * <p> Send a MySQL query</p>
   * 
   * 
   * @name maClass:query
   * @param void
   * @return void
   **/

  public function query($query);

  /**
   * real_escape_string
   * <p> Escapes special characters in a string for use in a SQL statement</p>
   * 
   * 
   * @name maClass:real_escape_string
   * @param void
   * @return void
   **/

  public function realEscapeString($string);

  /**
   * result
   * <p> Get result data</p>
   * 
   * 
   * @name maClass:result
   * @param void
   * @return void
   **/

  public function result($result,$row,$field=0);

  /**
   * select_db
   * <p> Select a MySQL database</p>
   * 
   * 
   * @name maClass:select_db
   * @param void
   * @return void
   **/

  public function selectDb($database_name);

  /**
   * set_charset
   * <p> Sets the client character set</p>
   * 
   * 
   * @name maClass:set_charset
   * @param void
   * @return void
   **/

  public function setCharset();

  /**
   * stat
   * <p> Get current system status</p>
   * 
   * 
   * @name maClass:stat
   * @param void
   * @return void
   **/

  public function stat();

  /**
   * tablename
   * <p> Get table name of field</p>
   * 
   * 
   * @name maClass:tablename
   * @param void
   * @return void
   **/

  public function tablename();

  /**
   * thread_id
   * <p> Return the current thread ID</p>
   * 
   * 
   * @name maClass:thread_id
   * @param void
   * @return void
   **/

  public function threadId();

  /**
   * unbuffered_query
   * <p> Send an SQL query to MySQL, without fetching and buffering the result rows</p>
   * 
   * 
   * @name maClass:unbuffered_query
   * @param void
   * @return void
   **/

  public function unbufferedQuery();
}
?>
