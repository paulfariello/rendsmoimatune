(function(app) {
	app.AppComponent = ng.core.Component({
		selector: 'my-app',
		templateUrl: '/home.html'
	}).Class({
		constructor: function() {}
	});
})(window.app || (window.app = {}));
