window.addEvent('domready', function()
{
    $$('.slide').each(function(slide)
    {
        slider = slide.getPrevious('.slider');
        if ($chk(slider)) {
            slide.set('slide');
            slide.slide('out');
            slider.addEvent('click', function()
            {
                this.slide();
            }.bind(slide));
        }
    });
});
