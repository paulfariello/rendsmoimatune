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

    if (typeof(event) == "undefined") {
        event.stop();
    }

    payer.getElement('.remove-payer').addEvent("click", deletePayer);
    payer.getElements('input[type!=button]').each(function(input)
    {
        input.set('value','');
    });
    autoCompletePayer(payer.getElement('input.payer-name'));
    autoAddBeneficiary(payer.getElement('input.payer-name'));


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
    
    payer.inject($('add-new-payer').getParent(), 'before');

    return payer;
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

function addBeneficiaryEvent(event)
{
    if (typeof(event) != "undefined") {
        event.stop();
    }

    addBeneficiary(null);
}

function addBeneficiary(user)
{

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
        autoCompleteBeneficiary(input);
    });

    if (user != null) {
        beneficiary.getElements('input.beneficiary-name').each(function(beneficiaryName)
        {
            beneficiaryName.set('value', user.name);
        });
        beneficiary.getElements('input[name^=beneficiariesId]').each(function(beneficiaryId)
        {
            beneficiaryId.set('value', user.id);
        });
    }

    beneficiary.inject($('add-new-beneficiary').getParent(), 'before');
}

function addAllParticipants(event)
{
    if (typeof(event) != "undefined") {
        event.stop();
    }

    var loadParticipants = new Element('div', {
        id: 'load-participants',
        class: 'ajax-loader'
    });

    loadParticipants.inject($('add-new-beneficiary').getParent(), 'before');

    var request = new Request.JSON({
        url: event.target.get('rel'),
        onSuccess: function(users) {
            var loadParticipants = $('load-participants');
            loadParticipants.destroy();
            users.each(function(user) {
                addBeneficiary(user);
            });
        },
        onFailure: function() {
            var loadParticipants = $('load-participants');
            loadParticipants.destroy();
        },
        onException: function() {
            var loadParticipants = $('load-participants');
            loadParticipants.destroy();
        },
    }).get(); 
}

function autoCompleteBeneficiary(input)
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

function autoCompletePayer(input)
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

function autoAddBeneficiary(input)
{
    input.addEvent('change', function(event)
    {
        payerName = event.target;
        payerId = payerName.getPrevious('input[name^=payersId]');
        cloned = false;
        $$('input.beneficiary-name').each(function(beneficiaryName)
        {
            beneficiaryId = beneficiaryName.getPrevious('input[name^=beneficiariesId]');
            if (!cloned && beneficiaryName.get('value') == payerName.get('value') && beneficiaryId.get('value') == payerId.get('value')) {
                cloned = true;
            } else if (!cloned && beneficiaryName.get('value') == '' && beneficiaryId.get('value') == '' && beneficiaryName.isVisible()) {
                beneficiaryName.set('value', payerName.get('value'));
                beneficiaryId.set('value', payerId.get('value'));
                cloned = true;
            }
        });

        if (!cloned) {
            user = { 
                id: payerId.get('value'),
                name: payerName.get('value'),
            };
            addBeneficiary(user);            
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
    $('add-new-beneficiary').addEvent("click", addBeneficiaryEvent);
    $('add-all-participants').addEvent("click", addAllParticipants);

    // REMOVE BENEFICIARY
    $$('.remove-beneficiary').each(function(button)
    {
        button.addEvent("click", deleteBeneficiary);
    });

    $$('input.payer-name').each(function(input)
    {
        // AUTO-COMPLETE PAYER
        autoCompletePayer(input);
        // AUTO-ADD BENEFICIARY
        autoAddBeneficiary(input);
    });

    $$('input.beneficiary-name').each(function(input)
    {
        // AUTO-COMPLETE BENEFICIARY
        autoCompleteBeneficiary(input);
    });

    // AUTO-COMPLETE PAYER AMOUNT
    $('expenditure-amount').addEvent('blur', function(event)
    {
        amount = event.target.get('value');
        $$('input.payer-amount').each(function(input)
        {
            if (input.get('value') == '' && input.isVisible()) {
                input.set('value', amount);
            }
        });
    });



});
