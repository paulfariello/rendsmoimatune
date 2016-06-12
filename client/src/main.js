import 'src/css/main.scss'
import $ from 'jquery'
import 'foundation-datepicker/js/foundation-datepicker.js'
import 'foundation-datepicker/css/foundation-datepicker.scss'
import Vue from 'vue'
import VueRouter from 'vue-router'
import VueResource from 'vue-resource'
import App from './app'
import Landing from './landing'
import Loading from './loading'
import Account from './account'
import AccountDetail from './account-detail'
import Expenditures from './expenditures'
import EditExpenditure from './edit-expenditure'
import ListExpenditures from './list-expenditures'
import EditRepayment from './edit-repayment'
import Repayments from './repayments'

Vue.use(VueRouter)
Vue.use(VueResource)

Vue.component('loading', Loading)
Vue.component('expenditures', Expenditures)
Vue.component('repayments', Repayments)

Vue.filter('currency', function (amount) {
	return Math.round(amount) * 1.0 / 100 + ' €'
})

Vue.filter('amount', {
	read: function (amount) {
		return Math.round(amount) * 1.0 / 100
	},
	write: function (amount) {
		amount = amount.replace(/,/g, '.')
		return isNaN(amount) ? 0 : Math.round(parseFloat(amount) * 100)
	}
})

Vue.directive('date-picker', {
	twoWay: true,
	params: ['format', 'language'],
	bind: function () {
		var self = this
		$(this.el).fdatepicker({
			initialDate: this.value,
			format: this.params.format,
			language: this.params.language
		}).on('changeDate', function (ev) {
			self.set(ev.date)
		})
	},
	update: function (newValue) {
		$(this.el).fdatepicker('update', newValue)
	}
})

Vue.http.options.root = '/api'

const router = new VueRouter({
	history: false,
	saveScrollPosition: true
})

router.map({
	'/': {
		component: Landing
	},
	'/account/:accountId': {
		name: 'account',
		component: Account,
		subRoutes: {
			'/': {
				component: AccountDetail
			},
			'/edit-expenditure': {
				name: 'edit-expenditure',
				component: EditExpenditure
			},
			'/expenditures': {
				name: 'expenditures',
				component: ListExpenditures
			},
			'/edit-repayment': {
				name: 'edit-repayment',
				component: EditRepayment
			}
		}
	}
})

router.start(App, 'body')
