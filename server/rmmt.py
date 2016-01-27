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
    DB.create_tables([Account, User])


class RmmtModel(peewee.Model):
    """Base class for all rmmt model classes"""
    class Meta:
        database = DB

    @property
    def json(self):
        """simple representation of the object that can feed json.dumps"""
        raise NotImplementedError


class Account(RmmtModel):
    """Account is the main document"""
    _id = peewee.PrimaryKeyField()
    uid = peewee.UUIDField()
    name = peewee.CharField()

    @property
    def json(self):
        users = []
        for user in self.users:
            users.append(user.json)
        return {"uid": uniqid.encode(self.uid),
                "name": self.name,
                "users": users}


class User(RmmtModel):
    """User participating in an account"""
    _id = peewee.PrimaryKeyField()
    name = peewee.CharField()
    account = peewee.ForeignKeyField(Account, related_name='users')

    @property
    def json(self):
        return {"name": self.name,
                "account": uniqid.encode(self.account.uid)}
