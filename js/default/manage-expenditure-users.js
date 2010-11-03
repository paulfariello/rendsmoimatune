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
    var euro = '\u20ac';
    var payer = $('clonable-payer').clone();

    event.stop();

    payer.getElement('.remove-payer').addEvent("click", deletePayer);
    payer.getElements('input[type!=button]').each(function(input)
    {
        input.set('value','');
    });
    autocompletePayer(payer.getElement('input.payer-name'));


    // Autocomplete amount and metric
    var expenditureAmount = parseFloat($('expenditure-amount').get('value'));
    var payedAmount       = 0;
    var newMetric         = '%';
    var newAmount;
    var total;


    $$('select[name^=payersMetric]').each(function(select) {
        var metric = select.get('value');
        switch(metric) {
            case '%':
                break;
            case euro:
                // If an input is in € then the new one will be in €
                newMetric = euro;
                break;
        }
    });


    switch(newMetric) {
        case '%':
            total = 100;
            break;
        case euro:
            // If an input is in € then the new one will be in €
            newMetric = euro;
            total      = expenditureAmount;
            break;
        default:
            total = null;
    }

    $$('input[name^=payersAmount]').each(function(input)
    {
        var amount = parseFloat(input.get('value'));
        var metric = input.getNext('select[name^=payersMetric]').get('value');
        if (! isNaN(amount)) {
            amount       = convertAmount(amount, metric, newMetric, expenditureAmount);
            payedAmount += amount;
        }
    });

    if (! isNaN(total)) {
        newAmount = total - payedAmount;
        payer.getElement('input[name^=payersAmount]').set('value', newAmount);
        payer.getElement('select[name^=payersMetric]').set('value', newMetric);
    }
    
    payer.inject(this.getParent(), 'before');
}

function convertAmount(amount, from, to, expenditureAmount)
{
    var euro = '\u20ac';
    if (from == to) {
        return amount;
    } else {
        if (from == '%' && to == euro) {
            return amount / 100 * expenditureAmount;
        } else if (from == euro && to == '%') {
            return amount / expenditureAmount * 100;
        }
    }

    return null;
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
        autocompleteBeneficiary(input);
    });
    beneficiary.inject(this.getParent(), 'before');
}

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
        autocompletePayer(input);
    });

    // AUTO-COMPLETE BENEFICIARY
    $$('input.beneficiary-name').each(function(input)
    {
        autocompleteBeneficiary(input);
    });

});