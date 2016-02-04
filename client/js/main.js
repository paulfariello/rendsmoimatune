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
			templateUrl: "templates/account.html",
			controller: "AccountCtrl"
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

	rmmt.filter("amount", function() {
		return function(amount) {
			return amount * 1. / 100 + " €";
		}
	});

	rmmt.controller("AccountCtrl", function($scope, $http) {
		$http.get("http://localhost:8080/api/account/XAjeAAbAE64JNRTMKbtBeD")
			.success(function(data) {
				$scope.account = data;
                $scope.account.max_debt = 0;
                for (var user in $scope.account.users) {
                    var balance = Math.abs($scope.account.users[user].balance);
                    if ($scope.account.max_debt < balance) {
                        $scope.account.max_debt = balance;
                    }
                }
			});
	});
})();
