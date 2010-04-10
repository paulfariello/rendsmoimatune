window.addEvent("domready", function()
{
  if($chk($('console-log'))) {

    /**
     * On cache tous les onglets
     */
    $$('div.console-tab').each(function(el)
      {
        el.set('slide',
        {
          mode: 'vertical',
          onComplete: function()
          {
            this.getParent().setStyle('height','auto');
          }.bind(el)
        });
        el.slide('hide');
      });

    /**
     * On ajoute les évènements au menu principale
     */

    $$('#console-menu > ul > li').each(function(el)
    {
      el.addEvent('click', function()
      {
        var id = this.getProperty('id').split('-')[2];
        $('console-tab-'+id).slide('toggle');
      });
    });

    /**
     * On ajoute les évènements au filtre de log
     */
    $$('.log-filter').each(function(el)
    {
      el.addEvent('click',function()
      {
        var filter = el.getProperty('id').split('-')[3];
        $$('#console-log-menu .selected').each(function(el)
        {
          el.removeClass('selected');
          var filter = el.getProperty('id').split('-')[3];
          if(filter == 'all') {
            $$('#console-log-content > .log').each(function(el)
            {
              el.setStyle('display','none');
            });
          } else {
            $$('#console-log-content > .'+filter).each(function(el)
            {
              el.setStyle('display','none');
            });
          }
        });
        el.addClass('selected');
        if(filter == 'all') {
          $$('#console-log-content > .log').each(function(el)
          {
            el.setStyle('display','block');
          });
        } else {
          $$('#console-log-content > .'+filter).each(function(el)
          {
            el.setStyle('display','block');
          });
        }
      });
    });

    /**
     * On ajoute les évènements au backtrace et log-title
     */

    $$('.backtrace-link').each(function(el)
    {
      el.addEvent('click',function()
      {
        this.getNext('.backtrace').toggleClass('hidden');
        this.getNext('.log-content').addClass('hidden');
      });
    });

    $$('.log-title').each(function(el)
    {
      el.addEvent('click',function()
      {
        this.getNext('.log-content').toggleClass('hidden');
        this.getNext('.backtrace').addClass('hidden');
      });
    });
  }
});
