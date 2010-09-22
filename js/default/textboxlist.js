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
    script = $('textboxlist-src');
    uri = script.get('src');
    uri = new URI(uri.substring(0,uri.lastIndexOf("/")+1)+'../../../../');
    root = uri.toAbsolute();
    
    $$('input.textboxuserlist').each(function(input)
    {
        var tbl = new TextboxList(input, {
            unique: true,
            bitsOptions: {
                editable: {
                    addKeys: 188
                }
            },
            plugins: {
                autocomplete: {
                    minLength: 3,
                    queryRemote: true,
                    remote: {url: root+'ajax/autocomplete-user.php'}
                }
            }
        });

        input.store('textboxlist', tbl);
    });
});
