<template>
<div class="row">
	<div class="small-12 columns">
		<form v-on:submit="saveExpenditure">
			<div class="row">
				<div class="small-12 columns">
					<label>Nom
						<input type="text" v-model="expenditure.name" required />
					</label>
				</div>
			</div>
			<div class="row">
				<div class="small-6 columns">
					<label>Montant
						<div class="input-group">
							<input type="text" v-model="expenditure.amount | amount" pattern="[0-9]+([.,][0-9]*)?" class="input-group-field" required />
							<span class="input-group-label">€</span>
						</div>
					</label>
					<p class="help-text">Montant en euro. Exemple: 42,5</p>
				</div>
				<div class="small-6 columns">
					<label>Date
						<input type="text" v-date-picker="expenditure.date" format="yyyy-mm-dd" language="fr" required />
					</label>
				</div>
			</div>
			<div class="row">
				<div class="small-6 columns">
					<label>Payeur
						<select v-model="expenditure.payer">
							<option v-for="user in account.users" value="{{ user.name }}">{{ user.name }}</option>
						</select>
					</label>
				</div>
				<div class="small-6 columns">
					<table>
						<thead>
							<tr>
								<th>
									<div class="switch">
										<input class="switch-input" type="checkbox" id="debt-all" v-model="allDebts">
										<label for="debt-all" class="switch-paddle">
											<span class="show-for-sr">Selectionner tous les participants</span>
										</label>
									</div>
								</th>
								<th>Participant</th>
								<th>Part</th>
							</tr>
						</thead>
						<tbody>
							<tr v-for="user in account.users">
								<input type="hidden" v-model="expenditure.debts[$index].debtor" :value="user.name" />
								<td>
									<div class="switch">
										<input class="switch-input" type="checkbox" v-model="expenditure.debts[$index].debt" id="debt-{{$index}}">
										<label for="debt-{{$index}}" class="switch-paddle">
											<span class="show-for-sr">{{ user.name }}</span>
										</label>
									</div>
								</td>
								<td>{{ user.name }}</td>
								<td>
									<input v-if="expenditure.debts[$index].debt" type="number" v-model="expenditure.debts[$index].share" number>
								</td>
							</tr>
						</tbody>
					</table>
				</div>
			</div>
			<div class="row">
				<div class="small-12 columns">
					<button type="submit" class="button fa fa-plus-circle">Enregistrer</button>
				</div>
			</div>
		</form>
	</div>
</div>
</template>

<script>
export default {
	data () {
		var debts = []
		for (var i in this.account.users) {
			debts.push({'debt': true, 'share': 1, 'debtor': this.account.users[i].name})
		}
		return {
			'allDebts': true,
			'expenditure': {
				'name': '',
				'amount': 0,
				'date': new Date(),
				'payer': '',
				'debts': debts
			}
		}
	},
	route: {
		data () {
			var i
			var debts = []
			for (i in this.account.users) {
				debts.push({'debt': true, 'share': 1, 'debtor': this.account.users[i].name})
			}
			if (typeof this.$route.params.expenditureId === 'undefined') {
				this.allDebts = true
				this.expenditure.name = ''
				this.expenditure.amount = 0
				this.expenditure.date = new Date()
				this.expenditure.payer = ''
				this.expenditure.debts = debts
			} else {
				var expenditureId = Number(this.$route.params.expenditureId)
				for (i in this.account.expenditures) {
					var expenditure = this.account.expenditures[i]
					if (expenditure.id === expenditureId) {
						for (var j in expenditure.debts) {
							expenditure.debts[j].debt = expenditure.debts[j].share > 0
						}
						this.allDebts = true
						this.expenditure = expenditure
						break
					}
				}
			}
		}
	},
	watch: {
		'allDebts': function (val) {
			for (var i in this.expenditure.debts) {
				this.expenditure.debts[i].debt = val
			}
		}
	},
	props: {
		account: {
			type: Object,
			required: true
		}
	},
	methods: {
		saveExpenditure () {
			for (var i in this.expenditure.debts) {
				var debt = this.expenditure.debts[i]
				if (!debt.debt) {
					debt.share = 0
				}
			}

			function success (response) {
				this.$router.go({name: 'account', params: { accountId: this.$route.params.accountId }})
			}

			function error (response) {
				// TODO error handling
			}

			var resource
			if (typeof this.$route.params.expenditureId === 'undefined') {
				resource = this.$resource('account/' + this.$route.params.accountId + '/expenditures/')
				resource.save(this.expenditure).then(success, error)
			} else {
				resource = this.$resource('account/' + this.$route.params.accountId + '/expenditures/' + this.$route.params.expenditureId)
				resource.update(this.expenditure).then(success, error)
			}
		}
	}
}
</script>
