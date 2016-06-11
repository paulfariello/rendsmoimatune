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
				<a ui-sref="account.edit-expenditure({expenditure_id: expenditure.id})" aria-label="Éditer" class="button"><i class="fa fa-pencil fa-lg"></i></a>
				<button aria-label="Supprimer" class="button alert"><i class="fa fa-trash-o fa-lg"></i></button>
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
			default: 5
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
		}
	}
}
</script>
