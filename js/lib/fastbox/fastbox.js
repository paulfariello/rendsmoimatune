var Fastbox = (function(){

        function Fastbox(links){
			
				var galery = $$(links);

                // Apply lightbox on the links elements
                galery.each(function(link){
                        link.addEvent('click', function(e){
								link = this;
                                e.stop();

                                // ===============
                                // = GLOBAL VARS =
                                // ===============
                                var img, loading, closed, timer, block, fix_hud, img_width, img_height;

                                // ===================
                                // = BUILD INTERFACE =
                                // ===================

                                // OVERLAY
                                if (!overlay) {
                                        var overlay = new Element('div', {'id': 'overlay'});
                                        // --- Effects
                                        var fx_o = new Fx.Morph(overlay, {'duration': 500, 'link': 'cancel'});
                                        // --- Events
                                        overlay.addEvent('click', function(){ close(); });
                                }
                                // --- Inject
                                fx_o.set({'opacity': 0});
                                overlay.inject(document.body);

                                // IMAGE FRAME
                                if (!frame) {
                                        var frame = new Element('div', {'id': 'frame'});
                                        // --- Events
                                        frame.addEvent('click', function(){ close(); });
                                }
                                // --- Inject
                                frame.inject(document.body);

                                // INDICATOR
                                if (!indicator){
                                        var indicator = new Element('div', {'id': 'indicator'});
                                        var busy = new Element('div', {'id': 'busy'}).inject(indicator);
                                        // --- Events
                                        indicator.addEvent('click', function(){ close(); });
                                }

                                // HUD CONTROLLER
                                if (!hud) {
                                        var c_prev = new Element('a', {'id': 'c_prev'});
                                        var c_down = new Element('a', {'id': 'c_down'});
                                        var c_next = new Element('a', {'id': 'c_next'});
                                        var controls = new Elements([c_prev, c_down, c_next]);
                                        var hud = new Element('div', {'id': 'hud'}).adopt(controls);
                                        // --- Effects
                                        var fx = new Fx.Morph(hud, {'duration': 500, 'link': 'cancel'});
                                        fx.addEvent('onComplete', function(){
                                                block = false;
                                                if (fix_hud) return;
                                                if (hud.getStyle('opacity') === 1) {
                                                        timer = fx.start.delay(1000, fx, {'opacity': 0});
                                                }
                                        });
                                        // --- Events
                                        controls.addEvent('mouseenter', function(){
                                                $clear(timer);
                                                fix_hud = true;
                                        })
                                        controls.addEvent('mouseout', function(){
                                                fix_hud = false;
                                                timer = fx.start.delay(1000, fx, {'opacity': 0});
                                        });
                                        c_next.addEvent('click', function(){ change_img('next', c_next); });
                                        c_prev.addEvent('click', function(){ change_img('prev', c_prev); });
                                }
                                // --- Inject
                                fx.set({'opacity': 0});
                                hud.inject(document.body);

                                // DOCUMENT EVENTS
                                window.addEvent('mousemove', function(){
                                        if (block) return;
                                        block = true;
                                        $clear(timer);
                                        fx.start({'opacity': 1});
                                });
                                document.addEvent('keydown', function(event){
                                        switch(event.code) {
                                                case 27:        // Esc
                                                case 88:        // 'x'
                                                case 67:        // 'c'
                                                close();
                                                break;
                                                case 40:        // down
                                                location = link.getProperty('href');
                                                break;
                                                case 37:        // Left arrow
                                                case 80:        // 'p'
                                                change_img('prev', c_prev);
                                                break;  
                                                case 39:        // Right arrow
                                                case 78:        // 'n'
                                                change_img('next', c_next);
                                        }
                                });

                                // ======================
                                // = INTERACTIVE EVENTS =
                                // ======================

                                // CLOSING EVENT
                                function close(){
                                        closed = true;
                                        if (indicator) indicator.dispose();
                                        frame.dispose();
                                        hud.dispose();
                                        fx_o.start({'opacity': 0}).chain(function(){
                                                overlay.dispose();
                                        });
                                        document.removeEvents('keydown');
                                        window.removeEvents('mousemove');
                                        window.removeEvents('resize');
                                }

                                // UPDATE HUD, AND PRELOAD
                                function update(){
                                        // Test prev/next images
                                        if (galery.indexOf(link) < galery.length - 1) {
                                                c_next.removeClass('inactive');
                                                // Preloading
                                                new Asset.image(galery[galery.indexOf(link)+1].getProperty('href'));
                                        }
                                        else {
                                                c_next.addClass('inactive');
                                        }
                                        if (galery.indexOf(link) > 0) {
                                                c_prev.removeClass('inactive');
                                                // Preloading
                                                new Asset.image(galery[galery.indexOf(link)-1].getProperty('href'));
                                        }
                                        else {
                                                c_prev.addClass('inactive');
                                        }
                                        // Set the HREF for the download controller
                                        c_down.setProperty('href', link.getProperty('href'));
                                }

                                // IMAGE IS READY
                                function img_ready(){
                                        loading = false;
                                        if (closed) return;
                                        // Timeout pour firefox qui fait un glitch visuel sinon
                                        setTimeout(function(){ indicator.dispose(); }, 100);
                                        img.inject(frame);
                                        img_width = img.getSize().x;
                                        img_height = img.getSize().y;
                                        resize_img();
                                }

                                // PREV/NEXT IMAGES CALL
                                function change_img(to, button){
                                        if (button.hasClass('inactive')) return;
                                        img.dispose();
                                        if (!$('busy')) indicator.grab(busy);
                                        indicator.removeClass('error').inject(document.body);
                                        // Update HUD
										
                                        if (to == "next") {
                                                link = galery[galery.indexOf(link)+1];
                                        }
                                        else {
                                                link = galery[galery.indexOf(link)-1];
                                        }
                                        update();
                                        // Do the asset now
                                        loading = true;
                                        img = new Asset.image(link.getProperty('href'), {
                                                'title': link.getProperty('title'),
                                                'alt': link.getProperty('title'),
                                                'onerror': function(){
                                                        indicator.empty().setProperty('class', 'error');
                                                },
                                                'onload': function(){
                                                        img_ready();
                                                }
                                        });
                                }

                                // RESIZE IMAGE
                                function resize_img(){
                                        // Get some informations
                                        var win_width = window.getSize().x;
                                        var win_height = window.getSize().y;

                                        var ratio = img_width / img_height;
                                        var height; var width;

                                        // Resizing function
                                        function resize(dimension){
                                                if (dimension == "x") {
                                                        width = win_width - 20;
                                                        height = width / ratio;
                                                }
                                                else {
                                                        height = win_height - 20;
                                                        width = height * ratio;
                                                }
                                        }

                                        // Let's do the tests
                                        if (win_width >= img_width) {
                                                // Pas tronquée
                                                if (win_height >= img_height) {
                                                        width = img_width;
                                                        height = img_height;
                                                }
                                                // Tronquée en hauteur
                                                else {
                                                        resize('y');
                                                }
                                        }
                                        else {
                                                // Tronquée en largeur
                                                if (win_height >= img_height) {
                                                        resize('x');
                                                }
                                                // Tronquée en hauteur & largeur
                                                else {
                                                        // Fenêtre en portrait
                                                        if (win_width < win_height) {
                                                                // Image en paysage ou en carré
                                                                if (ratio >= 1) {
                                                                        resize('x');
                                                                }
                                                                // Image en portrait
                                                                else if (ratio < 1) {
                                                                        // Image plus rapidement redimensionnée en hauteur qu'en largeur
                                                                        if (img_width - win_width > img_height - win_height) {
                                                                                resize('y');
                                                                        }
                                                                        // Inversement
                                                                        else {
                                                                                resize('x');
                                                                        }
                                                                }
                                                        }
                                                        // Fenêtre en paysage
                                                        else {
                                                                // Image en paysage
                                                                if (ratio > 1) {
                                                                        // Image plus rapidement redimensionnée en hauteur qu'en largeur
                                                                        if (img_width - win_width > img_height - win_height) {
                                                                                resize('x');
                                                                        }
                                                                        // Inversement
                                                                        else {
                                                                                resize('y');
                                                                        }
                                                                }
                                                                // Image en portrait ou en carré
                                                                else if (ratio <= 1) {
                                                                        resize('y');
                                                                }
                                                        }
                                                }
                                        }
                                        // POSITION : TOP & LEFT
                                        var left = (win_width - width) / 2;
                                        var top = (win_height - height) / 2;
                                        // SET THE STYLES
                                        img.setProperties({'width': width, 'height': height});
                                        img.set('style', 'top:'+top+'px;left:'+left+'px');
                                }

                                // ==================
                                // = FIRST SEQUENCE =
                                // ==================

                                // DEPLOY OVERLAY
                                fx_o.start({'opacity': 0.8}).chain(function(){
                                        if (!loading) return;
                                        indicator.inject(document.body);
                                });

                                // LOAD FIRST IMAGE
                                loading = true;
                                img = new Asset.image(this.getProperty('href'), {
                                        'title': this.getProperty('title'),
                                        'alt': this.getProperty('title'),
                                        'onerror': function(){
                                                indicator.empty().setProperty('class', 'error');
                                        },
                                        'onload': function(){
                                                img_ready();
                                                window.addEvent('resize', function(){
                                                        resize_img();
                                                });
                                        }
                                });

                                // UPDATE HUD
                                update();
                        });
                });
        }

        return Fastbox;  
})();

window.addEvent("domready",function()
{
	Fastbox(".fastbox");
	Fastbox(".fastbox-big");
});
