<?php
/* 
This file is part of Rendsmoimatune.

Rendsmoimatune is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Rendsmoimatune is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Rendsmoimatune.  If not, see <http://www.gnu.org/licenses/>.

 */

\Bdf\Core::getInstance()->getTemplatesEngine()->assign('currentUser',\Eu\Rmmt\User::getCurrentUser());
\Bdf\Core::getInstance()->getTemplatesEngine()->assign('currentLocale',\Bdf\Core::getInstance()->getLocale());
\Bdf\Core::getInstance()->getTemplatesEngine()->assign('siteName',\Bdf\Core::getInstance()->getConfig("site", "site_name"));

?>
