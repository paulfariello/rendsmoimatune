-- phpMyAdmin SQL Dump
-- version 3.1.2deb1ubuntu0.1
-- http://www.phpmyadmin.net
--
-- Host: localhost
-- Generation Time: Jul 25, 2009 at 04:08 PM
-- Server version: 5.0.75
-- PHP Version: 5.2.6-3ubuntu4.1

SET SQL_MODE="NO_AUTO_VALUE_ON_ZERO";

--
-- Database: `botteDeFoin`
--

-- --------------------------------------------------------

--
-- Table structure for table `%prefix%_Security`
--

CREATE TABLE IF NOT EXISTS `%prefix%_Security` (
  `id` int(11) NOT NULL auto_increment,
  `nom` varchar(10) NOT NULL,
  `parent` int(11) NOT NULL,
  PRIMARY KEY  (`id`),
  KEY `parent` (`parent`)
  ) ENGINE=MyISAM  DEFAULT CHARSET=latin1 AUTO_INCREMENT=4 ;

  --
  -- Dumping data for table `%prefix%_Security`
  --

  INSERT INTO `%prefix%_Security` (`id`, `nom`, `parent`) VALUES
  (1, 'all', 0),
  (2, 'auth', 1),
  (3, 'admin', 2);

  -- --------------------------------------------------------

  --
  -- Table structure for table `%prefix%_User`
  --

  CREATE TABLE IF NOT EXISTS `%prefix%_User` (
    `id` int(11) NOT NULL auto_increment,
    `email` varchar(255) NOT NULL,
    `password` varchar(255) NOT NULL,
    PRIMARY KEY  (`id`),
    UNIQUE KEY `email` (`email`)
    ) ENGINE=MyISAM  DEFAULT CHARSET=latin1 AUTO_INCREMENT=2 ;

    --
    -- Dumping data for table `%prefix%_User`
    --

    -- --------------------------------------------------------

    --
    -- Table structure for table `%prefix%_User_Right`
    --

    CREATE TABLE IF NOT EXISTS `%prefix%_User_Right` (
      `id_user` int(11) NOT NULL,
      `id_right` int(11) NOT NULL,
      PRIMARY KEY  (`id_user`,`id_right`),
      KEY `%prefix%_User_Right` (`id_right`)
      ) ENGINE=MyISAM DEFAULT CHARSET=latin1;

      ALTER TABLE `%prefix%_User_Right` ADD FOREIGN KEY (id_user) REFERENCES %prefix%_User (id);
      ALTER TABLE `%prefix%_User_Right` ADD FOREIGN KEY (id_right) REFERENCES %prefix%_Security (id);
      --
      -- Dumping data for table `%prefix%_User_Right`
      --

      -- --------------------------------------------------------

      --
      -- Table structure for table `user`
      --

      CREATE TABLE IF NOT EXISTS `user` (
        `id` int(11) NOT NULL,
        `skin` varchar(255) NOT NULL,
        UNIQUE KEY `id` (`id`)
        ) ENGINE=MyISAM DEFAULT CHARSET=latin1;

      ALTER TABLE `user` ADD FOREIGN KEY (id) REFERENCES %prefix%_User (id);
