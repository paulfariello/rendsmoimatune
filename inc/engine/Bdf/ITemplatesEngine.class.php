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
 * ITemplatesEngine
 *
 * @category Class
 * @package  Bdf
 * @author   Paul Fariello <paul.fariello@gmail.com>
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link     http://www.bottedefoin.net
 */

interface ITemplatesEngine
{
    /**
     * Lance la génération d'une page via un templates
     *
     * @param string $fileName Le nom du templates à afficher
     *
     * @return void
     */
    public function display($fileName);

    /**
     * Lance la génération d'une page via un templates et retourne le résultat au lieux de l'afficher sur la sortie standard
     *
     * @param string $fileName Le nom du templates à afficher
     *
     * @return void
     */
    public function fetch($fileName);

    /**
     * Change le skin à utiliser pour la génération des templates
     *
     * @param string $skinName L'identifiant du skin
     *
     * @return void
     */
    public function setSkin($skinName);

    /**
     * Initialize et configure le moteur de templates
     *
     * @return void
     */
    public function initialization();

    /**
     * Assigne une variable au moteur de templates
     *
     * @param string $name  Le nom de la variable à assigner
     * @param mixed  $value La variable à assigner
     *
     * @return void
     */
    public function assign($name,$value);

    /**
     * Assigne une variable par référence au moteur de templates
     *
     * Permet d'améliorer les performances mais si la variable change entre
     * l'assignation et la génération du templates la nouvelle valeur sera
     * utilisée au seins du templates.
     *
     * @param string $name   Le nom de la variable à assigner
     * @param mixed  &$value La variable à assigner
     *
     * @return void
     */
    public function assignByRef($name,&$value);
}
