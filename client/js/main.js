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

							for (var i in $scope.expenditure.debts) {
								var debt = $scope.expenditure.debts[i];
								debt.debt = debt.share > 0;
							}

							$scope.save_expenditure = function() {
								for (var i in $scope.expenditure.debts) {
									var debt = $scope.expenditure.debts[i];
									if (!debt.debt) {
										debt.share = 0;
									}
								}

								$http.put("/api/account/"+$scope.account.uid+"/expenditures/"+$scope.expenditure.id, $scope.expenditure).success(function(data) {
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
					controller: ['$state', '$scope', '$http',
						function($state, $scope, $http) {
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
								for (var i in $scope.expenditure.debts) {
									var debt = $scope.expenditure.debts[i];
									if (!debt.debt) {
										debt.share = 0;
									}
								}

								$http.post("/api/account/"+$scope.account.uid+"/expenditures/", $scope.expenditure).success(function(data) {
									$scope.account.expenditures.push(data);
									$state.go("account", {}, {reload: true});
								});
							};
						}
					]
				},
			}
		});

		$stateProvider.state('account.repayments', {
			url: "/repayments",
			templateUrl: "templates/repayments.html"
		});

		$stateProvider.state('account.add-repayment', {
			url: "/repayments/add",
			params: {
				payer: '',
				beneficiary: '',
				amount: 0.0,
			},
			views: {
				'': {
					templateUrl: "templates/repayments-edit.html",
					controller: ['$state', '$stateParams', '$scope', '$http',
						function($state, $stateParams, $scope, $http) {
							/* Init repayment with some default values */
							console.log($stateParams);
							var repayment = {};
							repayment.date = new Date();
							if ($stateParams.payer) {
								repayment.payer = $stateParams.payer;
							} else {
								repayment.payer = $scope.account.users[0].name;
							}

							if ($stateParams.beneficiary) {
								repayment.beneficiary = $stateParams.beneficiary;
							} else {
								repayment.beneficiary = $scope.account.users[1].name;
							}

							if ($stateParams.amount) {
								repayment.amount = $stateParams.amount;
							}

							$scope.repayment = repayment;

							$scope.save_repayment = function() {
								$http.post("/api/account/"+$scope.account.uid+"/repayments/", $scope.repayment).success(function(data) {
									$scope.account.repayments.push(data);
									$state.go("account", {}, {reload: true});
								});
							};
						}
					]
				},
			}
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

	rmmt.directive('expenditures', function() {
		return {
			require: 'ngModel',
			restrict: 'E',
			templateUrl: 'templates/expenditures-list.html',
			scope: {
				expenditures: '=src',
				limit: '=limit'
			}
		}
	});

	rmmt.directive('repayments', function() {
		return {
			require: 'ngModel',
			restrict: 'E',
			templateUrl: 'templates/repayments-list.html',
			scope: {
				repayments: '=src',
				limit: '=limit'
			}
		}
	});
})();
