import os
from pprint import pprint

from django.test import TestCase
from django.conf import settings
from django.urls import reverse
from django.contrib.auth.models import User
from rest_framework.authtoken.models import Token
from rest_framework.test import (
    APIClient,
    APITestCase,
    APIRequestFactory,
    force_authenticate,
)

from utility import read_txt


class OrderTest(TestCase):
    def setUp(self):

        self.user = User.objects.create(
            username="Tester", email="Tester@gmail.com", password="secret"
        )
        self.token = Token.objects.create(user=self.user)
        self.client = APIClient()
        self.client.credentials(HTTP_AUTHORIZATION=f"Token {self.token.key}")

        self.url = reverse("order_request:request")
        self.code = "005930"
        self.accNo = "8131214911"

    def test_buy_order(self):
        headers = {"Authorization": self.token}

        params = {
            "rqName": "test",
            "scrNo": "0000",
            "accNo": self.accNo,
            "orderType": 1,  # 신규매수
            "code": self.code,
            "qty": 1,
            "price": 0,
            "hogaType": "03",
            "originOrderNo": "",
        }

        response = self.client.post(self.url, data=params, headers=headers)

        data = response.json()
        pprint(data)
