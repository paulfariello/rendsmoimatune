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
function deletePayer(event)
{
    event.stop();
    this.getParent().getParent().destroy();
}

function deleteBeneficiary(event)
{
    event.stop();
    this.getParent().getParent().destroy();
}

function addPayer(event)
{
    event.stop();
    var payer = $('clonable-payer').clone();
    payer.getElements('.remove-payer').each(function(button)
    {
       button.addEvent("click", deletePayer)
    });
    payer.getElements('input[type!=button]').each(function(input)
    {
        input.set('value','');
    });
    payer.getElements('input.payer-name').each(function(input)
    {
        autocomplete(input);
    });
    payer.inject(this.getParent(), 'before');
}

function addBeneficiary(event)
{
    event.stop();
    var beneficiary = $('clonable-beneficiary').clone();
    beneficiary.getElements('.remove-beneficiary').each(function(button)
    {
       button.addEvent("click", deleteBeneficiary)
    });
    beneficiary.getElements('input[type!=button]').each(function(input)
    {
        input.set('value','');
    });
    beneficiary.getElements('input.beneficiary-name').each(function(input)
    {
        autocomplete(input);
    });
    beneficiary.inject(this.getParent(), 'before');
}

function autocomplete(input)
{
    new Meio.Autocomplete.Select(input, input.get('rel'),
    {
        valueField: input.getPrevious('input[name^=payersId]'),
        valueFilter: function(data){
            return data.identifier;
        },
        filter: {
            type: 'contains',
            path: 'value'
        }
    });
}

window.addEvent("domready", function()
{
    // ADD PAYER
    $('add-new-payer').addEvent("click", addPayer);

    // REMOVE PAYER
    $$('.remove-payer').each(function(button)
    {
        button.addEvent("click", deletePayer);
    });

    // ADD BENEFICIARY
    $('add-new-beneficiary').addEvent("click", addBeneficiary);

    // REMOVE BENEFICIARY
    $$('.remove-beneficiary').each(function(button)
    {
        button.addEvent("click", deleteBeneficiary);
    });

    // AUTO-COMPLETE PAYER
    $$('input.payer-name').each(function(input)
    {
        autocomplete(input);
    });

    // AUTO-COMPLETE BENEFICIARY
    $$('input.beneficiary-name').each(function(input)
    {
        autocomplete(input);
    });

});