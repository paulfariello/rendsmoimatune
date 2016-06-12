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
					<button type="submit" class="button fa fa-plus-circle">Ajouter</button>
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
		for (var user in this.account.users) {
			debts.push({'debt': true, 'share': 1, 'debtor': user.name})
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
			var resource = this.$resource('account/' + this.$route.params.accountId + '/expenditures/')

			for (var i in this.expenditure.debts) {
				var debt = this.expenditure.debts[i]
				if (!debt.debt) {
					debt.share = 0
				}
			}

			resource.save(this.expenditure).then(function (response) {
				this.$router.go({name: 'account', params: { accountId: this.$route.params.accountId }})
			}, function (response) {
				// TODO error handling
			})
		}
	}
}
</script>
