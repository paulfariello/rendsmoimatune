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
    $$('input[type="checkbox"][name="merge[]"]').each(function(input)
    {
        input.addEvent('change', function(event)
        {
            var checkedInputs = $$('input[type="checkbox"][name="merge[]"]:checked');
            while (checkedInputs.length > 2) {
                var broken;
                checkedInputs.each(function(input)
                {
                    if (!broken && input != this) {
                        input.checked = false;
                        broken = true;
                    }
                }.bind(this));
                checkedInputs = $$('input[type="checkbox"][name="merge[]"]:checked');
            } 
        }.bindWithEvent(input));

    });
});
