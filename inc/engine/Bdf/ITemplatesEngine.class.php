<?php

/**
 * Nom de la class
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

interface ITemplatesEngine {
  public function display($fileName);
  public function setSkin($skinName);
  public function initialization();
  public function assign($name,$value);
  public function assignByRef($name,&$value);
}

?>
