<template>
<div class="row">
	<div class="small-12 columns">
		<h2><a ui-sref="account"><i class="fa fa-bank fa-lg fa-fw"></i>{{ account.name }}</a></h2>
	</div>
</div>
<div class="row" v-if="account.users.length > 0">
	<div class="small-12 columns">
		<h3 v-if="account.balance.length > 0"><i class="fa fa-balance-scale fa-lg fa-fw"></i>Balance</h3>
		<h3 v-if="account.balance.length == 0"><i class="fa fa-users fa-lg fa-fw"></i>Utilisateurs</h3>
		<div class="balance">
			<div class="row text-center" v-for="user in account.users">
				<div class="small-5 columns">
					<div class="debt" v-if="user.balance < 0" style="width: {{ -user.balance/account.max_debt*100 }}%">
						{{ user.balance | amount }}
					</div>
				</div>
				<div ng-class="{'small-offset-5': user.balance >= 0}" class="small-2 columns">
					{{ user.name }}
				</div>
				<div class="small-5 columns">
					<div class="credit" v-if="user.balance > 0" style="width: {{ user.balance/account.max_debt*100 }}%">
						{{ user.balance | amount }}
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
<div class="row">
	<form ng-submit="add_user()">
		<div class="small-12 columns">
			<h4>Nouveau participant</h4>
			<div class="input-group">
				<input type="text" class="input-group-field" ng-model="account.new_user" required />
				<div class="input-group-button">
					<button type="submit" class="button fa fa-user-plus">Ajouter</button>
				</div>
			</div>
		</div>
	</form>
</div>
<div class="row" v-if="account.users.length > 0">
	<div class="small-12 columns">
		<h3><i class="fa fa-exchange fa-lg fa-fw"></i>Équilibrage</h3>
		<table v-if="account.balance.length > 0">
			<thead>
				<tr>
					<th>De</th>
					<th></th>
					<th>Montant</th>
					<th></th>
					<th>À</th>
					<th>Action</th>
				</tr>
			</thead>
			<tbody>
				<tr v-for="repayment in account.balance | orderBy: name">
					<td>{{ repayment.from }}</td>
					<td>doit</td>
					<td>{{ repayment.amount | amount }}</td>
					<td>à</td>
					<td>{{ repayment.to }}</td>
					<td><a ui-sref="account.add-repayment({payer: repayment.from, beneficiary: repayment.to, amount: repayment.amount})" class="fa fa-plus-circle button">Ajouter</a></td>
				</tr>
			</tbody>
		</table>
		<p v-if="account.balance.length == 0">
			Rien à faire, vous êtes à l'équilibre !
		</p>
	</div>
</div>
<div class="row" v-if="account.users.length > 0">
	<div class="small-12 columns">
		<h3><a ui-sref="account.expenditures"><i class="fa fa-credit-card fa-lg fa-fw"></i>Dépenses</a></h3>
		<expenditures limit="5" src="account.expenditures"></expenditures>
		<a v-if="account.expenditures.length > 5" ui-sref="account.expenditures">Et {{ account.expenditures.length - 5 }} <span v-if="account.expenditures.length > 6">autres…</span><span v-if="account.expenditures.length == 6">autre.</span></a>
		<a class="button float-right fa fa-plus-circle" ui-sref="account.add-expenditure">Nouvelle dépense</a>
	</div>
</div>
<div class="row" v-if="account.users.length > 1">
	<div class="small-12 columns">
		<h3><a ui-sref="account.repayments"><i class="fa fa-exchange fa-lg fa-fw"></i>Remboursements</a></h3>
		<repayments limit="5" src="account.repayments"></repayments>
		<a v-if="account.repayments.length > 5" ui-sref="account.repayments">Et {{ account.repayments.length - 5 }} <span v-if="account.repayments.length > 6">autres…</span><span v-if="account.repayments.length == 6">autre.</span></a>
		<a class="button float-right fa fa-plus-circle" ui-sref="account.add-repayment">Nouveau remboursement</a>
	</div>
</div>
</template>

<script>
export default {
	data () {
		return {
			'account': {
				'balance': [],
				'expenditures': [],
				'name': '',
				'repayments': [],
				'uid': '',
				'users': []
			}
		}
	},
	methods: {
		getAccount () {
			var resource = this.$resource('account{/id}')

			resource.get({id: this.$route.params.accountId}).then(function (response) {
				console.log(response.data)
				this.$set('account', response.data)
			}, function (response) {
				// TODO error handling
			})
		}
	},
	ready () {
		this.getAccount()
	}
}
</script>

<style>
</style>
