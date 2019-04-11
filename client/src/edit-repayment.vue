<template>
<div class="row">
	<div class="small-12 columns">
		<form v-on:submit="saveRepayment">
			<div class="row">
				<div class="small-6 columns">
					<label>Montant
						<div class="input-group">
							<input type="text" v-model="repayment.amount | amount" pattern="[0-9]+([.,][0-9]*)?" class="input-group-field" required />
							<span class="input-group-label">€</span>
						</div>
					</label>
					<p class="help-text">Montant en euro. Exemple: 42,5</p>
				</div>
				<div class="small-6 columns">
					<label>Date
						<input type="text" v-date-picker="repayment.date" format="yyyy-mm-dd" language="fr" required />
					</label>
				</div>
			</div>
			<div class="row">
				<div class="small-6 columns">
					<label>Payeur
						<select v-model="repayment.payer">
							<option v-for="user in account.users" value="{{ user.name }}">{{ user.name }}</option>
						</select>
					</label>
				</div>
				<div class="small-6 columns">
					<label>Bénéficiaire
						<select v-model="repayment.beneficiary">
							<option v-for="user in account.users" value="{{ user.name }}">{{ user.name }}</option>
						</select>
					</label>
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
		return {
			'repayment': {
				'amount': 0,
				'date': new Date(),
				'payer': '',
				'beneficiary': ''
			}
		}
	},
	route: {
		data () {
			if (typeof this.$route.params.repaymentId === 'undefined' && typeof this.$route.params.amount === 'undefined') {
				this.repayment.amount = 0
				this.repayment.date = new Date()
				this.repayment.payer = ''
				this.repayment.beneficiary = ''
			} else if (typeof this.$route.params.amount === 'undefined') {
				var repaymentId = Number(this.$route.params.repaymentId)
				for (var i in this.account.repayments) {
					var repayment = this.account.repayments[i]
					if (repayment.id === repaymentId) {
						this.repayment = repayment
						break
					}
				}
			} else {
				this.repayment.amount = Math.round(this.$route.params.amount)
				this.repayment.payer = this.$route.params.payer
				this.repayment.beneficiary = this.$route.params.beneficiary
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
		saveRepayment () {
			var resource = this.$resource('account/' + this.$route.params.accountId + '/repayments/')

			resource.save(this.repayment).then(function (response) {
				this.$router.go({name: 'account', params: { accountId: this.$route.params.accountId }})
			}, function (response) {
				// TODO error handling
			})
		}
	}
}
</script>
