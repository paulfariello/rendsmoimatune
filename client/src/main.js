import 'src/css/main.scss'
import Vue from 'vue'
import VueRouter from 'vue-router'
import VueResource from 'vue-resource'
import App from './app'
import Landing from './landing'
import Account from './account'
import AccountDetail from './account-detail'
import Expenditures from './expenditures'
import EditExpenditure from './edit-expenditure'
import Repayments from './repayments'

Vue.use(VueRouter)
Vue.use(VueResource)

Vue.component('expenditures', Expenditures)
Vue.component('repayments', Repayments)

Vue.filter('amount', function (amount) {
	return Math.round(amount) * 1.0 / 100 + ' €'
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
			}
		}
	}
})

router.start(App, 'body')
