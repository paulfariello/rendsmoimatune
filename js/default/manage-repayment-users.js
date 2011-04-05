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
function autocompleteBeneficiary(input)
{
    if ($chk(input)) {
        new Meio.Autocomplete.Select(input, input.get('rel'),
        {
            valueField: input.getPrevious('input[name^=beneficiariesId]'),
            valueFilter: function(data){
                return data.identifier;
            },
            filter: {
                type: 'contains',
                path: 'value'
            }
        });
    }
}

function autocompletePayer(input)
{
    if ($chk(input)) {
    }
}

window.addEvent("domready", function()
{
    $$('input.payer-name, input.beneficiary-name').each(function(input)
    {
        new Meio.Autocomplete.Select(input, input.get('rel'),
        {
            valueField: input.getPrevious('input[name$=id]'),
            valueFilter: function(data){
                return data.identifier;
            },
            filter: {
                type: 'contains',
                path: 'value'
            }
        });
    });

});
