<template>
<div class="row" v-if="account.users.length > 0">
	<div class="small-12 columns">
		<h3 v-if="account.balance.length > 0"><i class="fa fa-balance-scale fa-lg fa-fw"></i>Balance</h3>
		<h3 v-if="account.balance.length == 0"><i class="fa fa-users fa-lg fa-fw"></i>Utilisateurs</h3>
		<div class="balance">
			<div class="row text-center" v-for="user in account.users | orderBy name">
				<div class="small-5 columns">
					<div class="debt" v-if="user.balance < 0" v-bind:style="{width: (-user.balance) / maxBalance(account) * 100 + '%'}">
						{{ user.balance | currency }}
					</div>
				</div>
				<div v-bind:class="[user.balance >= 0 ? 'small-offset-5' : '']" class="small-2 columns">
					{{ user.name }}
				</div>
				<div class="small-5 columns">
					<div class="credit" v-if="user.balance > 0" v-bind:style="{width: user.balance / maxBalance(account) * 100 + '%'}">
						{{ user.balance | currency }}
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
<div class="row">
	<form v-on:submit="addUser">
		<div class="small-12 columns">
			<h4>Nouveau participant</h4>
			<div class="input-group">
				<input type="text" class="input-group-field" v-model="new_user" required :disabled="adding_user"/>
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
				<tr v-for="repayment in account.balance | orderBy name">
					<td>{{ repayment.from }}</td>
					<td>doit</td>
					<td>{{ repayment.amount | currency }}</td>
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
		<expenditures :limit="5" :src="account.expenditures"></expenditures>
		<a v-if="account.expenditures.length > 5" v-link="{ name: 'expenditures' }">Et {{ account.expenditures.length - 5 }} <span v-if="account.expenditures.length > 6">autres…</span><span v-if="account.expenditures.length == 6">autre.</span></a>
		<a class="button float-right fa fa-plus-circle" v-link="{ name: 'create-expenditure' }">Nouvelle dépense</a>
	</div>
</div>
<div class="row" v-if="account.users.length > 1">
	<div class="small-12 columns">
		<h3><a ui-sref="account.repayments"><i class="fa fa-exchange fa-lg fa-fw"></i>Remboursements</a></h3>
		<repayments :limit="5" :src="account.repayments"></repayments>
		<a v-if="account.repayments.length > 5"  v-link="{ name: 'repayments' }">Et {{ account.repayments.length - 5 }} <span v-if="account.repayments.length > 6">autres…</span><span v-if="account.repayments.length == 6">autre.</span></a>
		<a class="button float-right fa fa-plus-circle" v-link="{ name: 'edit-repayment' }">Nouveau remboursement</a>
	</div>
</div>
</template>

<script>
export default {
	props: {
		account: {
			type: Object,
			required: true
		},
		adding_user: false
	},
	methods: {
		addUser () {
			var resource = this.$resource('account/' + this.$route.params.accountId + '/users/{name}')
			this.adding_user = true

			resource.save({name: this.new_user}).then(function (response) {
				this.account.users.push({name: response.data.name, balance: response.data.balance})
				this.new_user = ''
				this.adding_user = false
			}, function (response) {
				// TODO error handling
			})
		},
		maxBalance (account) {
			var max = 0
			for (var i in account.users) {
				var balance = Math.abs(account.users[i].balance)
				if (balance > max) {
					max = balance
				}
			}
			return max
		}
	}
}
</script>

<style>
</style>
