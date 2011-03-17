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
window.addEvent("domready", function()
{
    Asset.javascript("http://connect.facebook.net/en_US/all.js");
    /**
     * Not functionnal...
     *
     * $$('a.facebook-authentication').each(function(button)
     * {    
     *     button.addEvent('click', function(event)
     *     {
     *         event.stop();
     *         popup = window.open(this.get('href'), "Facebook Authentication", "location=1,status=1,scrollbars=1,width=400,height=400");
     *     }.bindWithEvent(button));
     * });
     */
});
