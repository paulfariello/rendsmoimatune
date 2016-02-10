(function() {
	'use strict';

	var rmmt = angular.module('rmmt', [
		'ui.router'
	]);

	rmmt.config(function($stateProvider, $urlRouterProvider) {
		$urlRouterProvider.otherwise("/");

		$stateProvider.state('home', {
			url: "/",
			templateUrl: "templates/home.html",
			controller: ['$state', '$scope', '$http', function($state, $scope, $http) {
				$scope.create_account = function() {
					var account = {}
					account.name = $scope.account_name;
					$http.post("/api/account/", account).success(function(data) {
						$state.go('account', {account_id: data.uid}, {reload: true});
					});
				}
			}]
		});

		$stateProvider.state('error', {
			url: "/error",
			templateUrl: "templates/error.html",
			params: { error: null },
			controller: ['$state', '$scope', '$stateParams',
				function($state, $scope, $stateParams) {
					$scope.error = $stateParams.error;
				}
			]
		});

		$stateProvider.state('account', {
			url: "/{account_id:[a-zA-Z0-9-_]+}",
			templateUrl: "templates/account.html",
			resolve: {
				'request': function($http, $stateParams) {
					return $http.get("/api/account/"+$stateParams.account_id).then(function(request) {
						var data = request.data;
						data.max_debt = 0;
						for (var user in data.users) {
							var balance = Math.abs(data.users[user].balance);
							if (data.max_debt < balance) {
								data.max_debt = balance;
							}
						}
						return request;
					}, function(request) {
						return request;
					});
				}
			},
			controller: ['$state', '$scope', '$http', 'request',
				function($state, $scope, $http, request) {
					if (request.status != 200) {
						$state.go("error", {error: request.data.error});
					}

					$scope.account = request.data;
					$scope.add_user = function() {
						var user = {}
						user.name = $scope.account.new_user;
						$http.post("/api/account/"+$scope.account.uid+"/users/", user).success(function(data) {
							$scope.account.users.push({name: data.name, balance: data.balance});
							$scope.account.new_user = "";
						});
					}
				}
			]
		});

		$stateProvider.state('account.expenditures', {
			url: "/expenditures",
			views: {
				'': {
					templateUrl: "templates/expenditures.html"
				}
			}
		});

		$stateProvider.state('account.edit-expenditure', {
			url: "/expenditures/{expenditure_id:[0-9]+}/edit",
			views: {
				'': {
					templateUrl: "templates/expenditures-edit.html",
					controller: ['$state', '$scope', '$http', '$stateParams',
						function($state, $scope, $http, $stateParams) {
							for (var expenditure in $scope.account.expenditures) {
								if ($scope.account.expenditures[expenditure].id == $stateParams.expenditure_id) {
									$scope.expenditure = $scope.account.expenditures[expenditure];
								}
							}

							$scope.save_expenditure = function() {
								var expenditure = {};
								expenditure.date = $scope.date;
								expenditure.name = $scope.name;
								expenditure.payer = $scope.payer;
								expenditure.amount = $scope.amount;
								expenditure.debts = [];
								for (var debtor in $scope.debtors) {
									if ($scope.debtors[debtor].debt) {
										expenditure.debts.push({debtor: debtor, share: $scope.debtors[debtor].share});
									}
								}
								$http.post("/api/account/"+$scope.account.uid+"/expenditures/", expenditure).success(function(data) {
									$scope.account.expenditures.push(data);
									$state.go("account", {}, {reload: true});
								});
							};
						}
					]
				},
			}
		});

		$stateProvider.state('account.add-expenditure', {
			url: "/expenditures/add",
			views: {
				'': {
					templateUrl: "templates/expenditures-edit.html",
					controller: ['$state', '$scope', '$http', function($state, $scope, $http) {
						/* Init expenditure with some default values */
						var expenditure = {};
						expenditure.date = new Date();
						expenditure.payer = $scope.account.users[0].name;
						expenditure.debts = [];
						for (var user in $scope.account.users) {
							var debt = {
								debt: true,
								debtor: $scope.account.users[user].name,
								share: 1
							};
							expenditure.debts.push(debt);
						}
						$scope.expenditure = expenditure;

						$scope.save_expenditure = function() {
							var expenditure = {};
							expenditure.date = $scope.expenditure.date;
							expenditure.name = $scope.expenditure.name;
							expenditure.payer = $scope.expenditure.payer;
							expenditure.amount = $scope.expenditure.amount;
							expenditure.debts = [];
							for (var debtor in $scope.expenditure.debts) {
								if ($scope.expenditure.debts[debtor].debt) {
									var debt ={
										debtor: $scope.expenditure.debts[debtor].debtor,
										share: $scope.expenditure.debts[debtor].share
									};
									expenditure.debts.push(debt);
								}
							}
							$http.post("/api/account/"+$scope.account.uid+"/expenditures/", expenditure).success(function(data) {
								$scope.account.expenditures.push(data);
								$state.go("account", {}, {reload: true});
							});
						};
					}]
				},
			}
		});

		$stateProvider.state('account.repayments', {
			url: "/repayments",
			templateUrl: "templates/repayments.html"
		});

		$stateProvider.state('account.repayments.add', {
			url: "/add",
			templateUrl: "templates/repayments-add.html"
		});

		$stateProvider.state('account.balance', {
			url: "/balance",
			templateUrl: "templates/balance.html"
		});
	});

	rmmt.filter("amount", function() {
		return function(amount) {
			return Math.round(amount) * 1. / 100 + " €";
		}
	});

	rmmt.filter("notIn", function($filter) {
		return function(array, filter, indexArray, indexFilter) {
			if (typeof(indexFilter) === "undefined") {
				indexFilter = indexArray;
			}

			return $filter("filter")(array, function(elem) {
				for (var i in filter) {
					if (filter[i][indexFilter] == elem[indexArray]) {
						return false;
					}
				}
				return true;
			});
		}
	});

	rmmt.directive('datepicker', function() {
		return {
			require: 'ngModel',
			restrict: 'A',
			scope: {
				format: "@",
				language: "@"
			},
			link: function($scope, element, attrs, ngModel) {
				$(element).fdatepicker({
					format: $scope.format,
					language: $scope.language
				});

				function toUser(date) {
					date = new Date(date);
					return $.fn.fdatepicker.DPGlobal.formatDate(date, $.fn.fdatepicker.DPGlobal.parseFormat($scope.format), $scope.language);
				}

				ngModel.$formatters.push(toUser);
			}
		}
	});

	rmmt.directive('amount', function() {
		return {
			require: 'ngModel',
			restrict: 'A',
			link: function(scope, element, attr, ngModel) {
				function fromUser(text) {
					    return parseInt(parseFloat(text) * 100);
				}

				function toUser(amount) {
					    return amount / 100;
				}

				ngModel.$parsers.push(fromUser);
				ngModel.$formatters.push(toUser);
			}
		}
	});
})();