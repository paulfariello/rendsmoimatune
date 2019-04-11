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
from urllib.parse import urlparse

import uniqid

DB = peewee.Proxy()

def atomic(f):
    def wrapper(*args):
        with DB.atomic():
            return f(*args)
    return wrapper

def connect(url):
    """Connect rmmt to the database

    :param url: the database scheme
    :type url: str
    """
    args = {}
    parsed = urlparse(url)
    if 'sqlite' in parsed.scheme:
        args['pragmas'] = (('foreign_keys', 'ON'),)
    DB.initialize(playhouse.db_url.connect(url, **args))


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
    def balance(self):
        creditors = []
        debitors = []
        for user in self.users:
            snapshot = {"user": user.name, "balance": user.balance}
            if user.balance > 0:
                creditors.append(snapshot)
            if user.balance < 0:
                debitors.append(snapshot)

        repayments = []
        while len(creditors) and len(debitors):
            creditors.sort(key=lambda creditor: creditor["user"])
            debitors.sort(key=lambda creditor: creditor["user"])

            creditor = creditors.pop()
            debitor = debitors.pop()
            amount = min(abs(creditor["balance"]), abs(debitor["balance"]))
            repayments.append({"from": debitor["user"], "to": creditor["user"],
                               "amount": amount})
            debitor["balance"] += amount
            creditor["balance"] -= amount

            assert(debitor["balance"] <= 0)
            assert(creditor["balance"] >= 0)

            if debitor["balance"] != 0:
                debitors.append(debitor)

            if creditor["balance"] != 0:
                creditors.append(creditor)

        return repayments


    @property
    def json(self):
        return {"uid": uniqid.encode(self.uid),
                "name": self.name,
                "users": [{"name": user.name, "balance": user.balance} for user in self.users],
                # TODO remove account from expenditure and repayment
                "expenditures": [expenditure.json for expenditure in self.expenditures],
                "repayments": [repayment.json for repayment in self.repayments],
                "balance": self.balance}


class User(RmmtModel, JSONObject):
    """User participating in an account"""
    _id = peewee.PrimaryKeyField()
    account = peewee.ForeignKeyField(Account, related_name="users", on_delete="CASCADE")
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
    account = peewee.ForeignKeyField(Account, related_name="expenditures", on_delete="CASCADE")
    name = peewee.CharField()
    date = peewee.DateField()
    amount = peewee.IntegerField()
    payer = peewee.ForeignKeyField(User, related_name="expenditures", on_delete="RESTRICT")

    @property
    def shares(self):
        shares = 0
        for debt in self.debts:
            shares += debt.share
        return shares

    @property
    def json(self):
        # rebuild debts for *all* users
        # this way it's easier to handler client side
        debts = []
        for user in self.account.users:
            debts.append({"debtor": user.name,
                          "share": 0})
            for debt in self.debts:
                if debt.debtor == user:
                    debts[-1]["share"] = debt.share

        return {"id": self._id,
                "account": uniqid.encode(self.account.uid),
                "name": self.name,
                "date": self.date.isoformat(),
                "amount": self.amount,
                "payer": self.payer.name,
                "shares": self.shares,
                "debts": debts}


class Debt(RmmtModel):
    """Association between an expenditure and user that are concerned by it"""
    _id = peewee.PrimaryKeyField()
    debtor = peewee.ForeignKeyField(User, related_name='debts', on_delete="RESTRICT")
    expenditure = peewee.ForeignKeyField(Expenditure, related_name='debts', on_delete="CASCADE")
    share = peewee.IntegerField()


class Repayment(RmmtModel, JSONObject):
    """One user directly give monney to another one"""
    _id = peewee.PrimaryKeyField()
    account = peewee.ForeignKeyField(Account, related_name="repayments", on_delete="CASCADE")
    date = peewee.DateField()
    amount = peewee.IntegerField()
    payer = peewee.ForeignKeyField(User, related_name="repayments_from_me", on_delete="RESTRICT")
    beneficiary = peewee.ForeignKeyField(User, related_name="repayments_to_me", on_delete="RESTRICT")

    @property
    def json(self):
        return {"id": self._id,
                "account": uniqid.encode(self.account.uid),
                "date": self.date.isoformat(),
                "amount": self.amount,
                "payer": self.payer.name,
                "beneficiary": self.beneficiary.name}
