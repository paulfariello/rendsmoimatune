#
# Copyright (c) 2016 Paul Fariello <paul@fariello.eu>
#
# Permission to use, copy, modify, and distribute this software for any
# purpose with or without fee is hereby granted, provided that the above
# copyright notice and this permission notice appear in all copies.
#
# THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
# WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
# MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
# ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
# WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
# ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
# OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
"""Rendsmoimatune multiuser account management"""

import peewee
import playhouse.db_url

import uniqid

DB = peewee.Proxy()


def connect(scheme):
    """Connect rmmt to the database

    :param scheme: the database scheme
    :type scheme: str
    """
    DB.initialize(playhouse.db_url.connect(scheme))


def create_tables():
    """Create required table if necessary"""
    DB.create_tables([Account, User, Expenditure, Debt, Repayment])


class JSONObject():
    """JSON object that can be dumped into raw json"""
    @property
    def json(self):
        """simple representation of the object that can feed json.dumps"""
        raise NotImplementedError


class RmmtModel(peewee.Model):
    """Base class for all rmmt model classes"""
    class Meta:
        database = DB


class Account(RmmtModel, JSONObject):
    """Account is the main document"""
    _id = peewee.PrimaryKeyField()
    uid = peewee.UUIDField()
    name = peewee.CharField()

    @property
    def json(self):
        return {"uid": uniqid.encode(self.uid),
                "name": self.name,
                "users": [{"name": user.name, "balance": user.balance} for user in self.users],
                # TODO remove account from expenditure and repayment
                "expenditures": [expenditure.json for expenditure in self.expenditures],
                "repayments": [repayment.json for repayment in self.repayments]}


class User(RmmtModel, JSONObject):
    """User participating in an account"""
    _id = peewee.PrimaryKeyField()
    account = peewee.ForeignKeyField(Account, related_name="users")
    name = peewee.CharField()

    @property
    def balance(self):
        total_payed = 0
        for expenditure in self.expenditures:
            total_payed += expenditure.amount
        for repayment in self.repayments_from_me:
            total_payed += repayment.amount

        total_debt = 0
        for debt in self.debts:
            total_debt += debt.expenditure.amount * debt.share / debt.expenditure.shares
        for repayment in self.repayments_to_me:
            total_debt += repayment.amount

        return total_payed - total_debt

    @property
    def json(self):
        return {"account": uniqid.encode(self.account.uid),
                "name": self.name,
                "expenditures": [{"name": expenditure.name,
                                  "date": expenditure.date.isoformat(),
                                  "amount": expenditure.amount} for expenditure in self.expenditures],
                "debts": [{"expenditure": debt.expenditure.name,
                           "date": debt.expenditure.date.isoformat(),
                           "amount": debt.expenditure.amount,
                           "shares": debt.expenditure.shares,
                           "share": debt.share} for debt in self.debts],
                "repayments_from_me": [{"date": repayment.date.isoformat(),
                                        "amount": repayment.amount,
                                        "to_user": repayment.to_user} for repayment in self.repayments_from_me],
                "repayments_to_me": [{"date": repayment.date.isoformat(),
                                        "amount": repayment.amount,
                                        "from_user": repayment.from_user} for repayment in self.repayments_to_me],
                "balance": self.balance}


class Expenditure(RmmtModel, JSONObject):
    """Expenditure set for a given account"""
    _id = peewee.PrimaryKeyField()
    account = peewee.ForeignKeyField(Account, related_name="expenditures")
    name = peewee.CharField()
    date = peewee.DateField()
    amount = peewee.IntegerField()
    payer = peewee.ForeignKeyField(User, related_name="expenditures")

    @property
    def shares(self):
        shares = 0
        for debt in self.debts:
            shares += debt.share
        return shares

    @property
    def json(self):
        return {"account": uniqid.encode(self.account.uid),
                "name": self.name,
                "date": self.date.isoformat(),
                "amount": self.amount,
                "payer": self.payer.name,
                "shares": self.shares,
                "debts": [{"debtor": debt.debtor.name,
                           "share": debt.share} for debt in self.debts]}


class Debt(RmmtModel):
    """Association between an expenditure and user that are concerned by it"""
    _id = peewee.PrimaryKeyField()
    debtor = peewee.ForeignKeyField(User, related_name='debts')
    expenditure = peewee.ForeignKeyField(Expenditure, related_name='debts')
    share = peewee.IntegerField()


class Repayment(RmmtModel, JSONObject):
    """One user directly give monney to another one"""
    _id = peewee.PrimaryKeyField()
    account = peewee.ForeignKeyField(Account, related_name="repayments")
    date = peewee.DateField()
    amount = peewee.IntegerField()
    from_user = peewee.ForeignKeyField(User, related_name="repayments_from_me")
    to_user = peewee.ForeignKeyField(User, related_name="repayments_to_me")

    @property
    def json(self):
        return {"account": uniqid.encode(self.account.uid),
                "date": self.date.isoformat(),
                "amount": self.amount,
                "from_user": self.from_user.name,
                "to_user": self.to_user.name}
