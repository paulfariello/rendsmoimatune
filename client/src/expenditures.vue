<template>
<table v-if="src.length > 0">
	<thead>
		<tr>
			<th>Date</th>
			<th>Nom</th>
			<th>Montant</th>
			<th>Payeur</th>
			<th>Participants</th>
			<th>Actions</th>
		</tr>
	</thead>
	<tbody>
		<tr v-for="expenditure in src | orderBy 'date' -1 | limitBy limit">
			<td>{{ expenditure.date }}</td>
			<td>{{ expenditure.name }}</td>
			<td>{{ expenditure.amount | currency }}</td>
			<td>{{ expenditure.payer }}</td>
			<td v-if="debtors(expenditure.debts).length <= expenditure.debts.length / 2">
				<span v-for="debt in debtors(expenditure.debts)">
					{{ debt.debtor }}{{$index+1 == debtors(expenditure.debts).length?'':', '}}
				</span>
			</td>
			<td v-if="debtors(expenditure.debts).length > expenditure.debts.length / 2">
				Tous
				<span v-if="notDebtors(expenditure.debts).length > 0">
					sauf
				</span>
				<span v-for="debt in notDebtors(expenditure.debts)">
					{{ debt.debtor }}{{$index+1 == notDebtors(expenditure.debts).length?'':', '}}
				</span>
			</td>
			<td>
				<a v-link="{ name: 'edit-expenditure', params: { expenditureId: expenditure.id } }" aria-label="Éditer" class="button"><i class="fa fa-pencil fa-lg"></i></a>
				<button aria-label="Supprimer" v-reveal-open="'delete-expenditure-modal-' + expenditure.id" class="button alert"><i class="fa fa-trash-o fa-lg"></i></button>
				<div class="reveal" id="delete-expenditure-modal-{{ expenditure.id }}" v-reveal-data>
					<h1>Supprimer {{ expenditure.name }} ?</h1>
					<p class="lead">Êtes-vous sûr de vouloir supprimer définitivement la dépense « {{ expenditure.name }} » ?</p>
					<button aria-label="Confirmer la suppression" v-reveal-close v-on:click="deleteExpenditure(expenditure)" class="button alert">Oui, je confirme</button>
					<button aria-label="Confirmer la suppression" v-reveal-close class="button">Non</button>
				</div>
			</td>
		</tr>
	</tbody>
</table>
</template>

<script>
export default {
	props: {
		limit: {
			type: Number,
			default: Infinity
		},
		src: {
			type: Array,
			required: true
		}
	},
	methods: {
		debtors (debts) {
			var debtors = []
			for (var i in debts) {
				if (debts[i].share > 0) {
					debtors.push(debts[i])
				}
			}
			return debtors
		},
		notDebtors (debts) {
			var notDebtors = []
			for (var i in debts) {
				if (debts[i].share === 0) {
					notDebtors.push(debts[i])
				}
			}
			return notDebtors
		},
		deleteExpenditure (expenditure) {
			var self = this
			var resource = this.$resource('account/' + this.$route.params.accountId + '/expenditures/' + expenditure.id)
			resource.delete(this.expenditure).then(function (response) {
				self.src.$remove(expenditure)
			}, function (response) {
				// TODO error handling
			})
		}
	}
}
</script>
