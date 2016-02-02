(function() {
	'use strict';

	var rmmt = angular.module('rmmt', [
		'ui.router'
	]);

	rmmt.config(function($stateProvider, $urlRouterProvider) {
		$urlRouterProvider.otherwise("/");

		$stateProvider.state('home', {
			url: "/",
			templateUrl: "templates/home.html"
		})

		$stateProvider.state('account', {
			url: "/{account_id:[a-zA-Z0-9-_]+}/",
			templateUrl: "templates/expenditures.html"
		})

		$stateProvider.state('expenditures', {
			url: "/{account_id:[a-zA-Z0-9-_]+}/expenditures/",
			templateUrl: "templates/expenditures.html"
		})

		$stateProvider.state('repayments', {
			url: "/{account_id:[a-zA-Z0-9-_]+}/repayments/",
			templateUrl: "templates/repayments.html"
		})

		$stateProvider.state('balance', {
			url: "/{account_id:[a-zA-Z0-9-_]+}/balance/",
			templateUrl: "templates/balance.html"
		})
	});
})();
