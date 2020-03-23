import os
from pprint import pprint

from django.test import TestCase
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


class TrRequestTests(TestCase):
    def setUp(self):
        self.user = User.objects.create(
            username="Tester", email="Tester@gmail.com", password="secret"
        )
        self.token = Token.objects.create(user=self.user)

        self.url = reverse("tr_request:request")
        self.client = APIClient()
        self.client.credentials(HTTP_AUTHORIZATION=f"Token {self.token.key}")

        self.code = "005930"
        self.date = "20200314"
        self.accNo = "8131214911"

    def test_OPT10004_success(self):
        """ 정상 요청 (OPT10004) """

        params = {
            "trCode": "OPT10004",
            "종목코드": self.code,
        }

        response = self.client.get(self.url, data=params)
        data = response.json()

        self.assertEqual(response.status_code, 200)
        self.assertGreater(len(data.get("멀티데이터")), 0)

    def test_OPT10004_no_auth_token(self):
        """ Auth Token을 request header에 추가하지 않은 경우 """

        params = {
            "trCode": "OPT10004",
            "종목코드": self.code,
        }

        self.client.credentials()
        response = self.client.get(self.url, data=params)
        data = response.json()

        self.assertEqual(response.status_code, 401)
        self.assertEqual(
            data.get("detail"), "자격 인증데이터(authentication credentials)가 제공되지 않았습니다."
        )

    def test_OPT10004_bad_code(self):
        """ 종목코드가 잘못된 경우 """
        params = {
            "trCode": "OPT10004",
            "종목코드": "dfghedbhweirtbnosamndfokansdfo",
        }

        response = self.client.get(self.url, data=params)
        data = response.content.decode("utf-8")

        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, "Bad Request")

    def test_OPT10004_bad_trcode(self):
        """ trCode가 잘못된 경우 """

        params = {
            "trCode": "ertidsnfpawenbtihsabdj",
            "종목코드": self.code,
        }

        response = self.client.get(self.url, data=params)
        data = response.content.decode("utf-8")

        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, "Bad Request")

    def test_OPT10004_bad_all(self):
        """ 종목코드와 trCode가 모두 잘못된 경우 """

        params = {
            "trCode": "ertidsnfpawenbtihsabdj",
            "종목코드": "dsfgbewrtiuewnrgdfsngsod",
        }

        response = self.client.get(self.url, data=params)
        data = response.content.decode("utf-8")

        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, "Bad Request")

    def test_OPT10005_success(self):

        params = {"trCode": "OPT10005", "종목코드": self.code}

        response = self.client.get(self.url, data=params)
        data = response.json()

        self.assertEqual(response.status_code, 200)
        self.assertGreater(len(data), 0)

    def test_OPT10059_success(self):

        params = {
            "trCode": "OPT10059",
            "일자": self.date,
            "종목코드": self.code,
            "금액수량구분": "1",  # 1:금액, 2:수량
            "매매구분": "0",  # 0:순매수, 1:매수, 2:매도
            "단위구분": "1",  # 1:단주, 1000:천주
        }

        response = self.client.get(self.url, data=params)
        data = response.json()

        self.assertEqual(response.status_code, 200)
        self.assertGreater(len(data), 0)

    def test_OPT10074_success(self):

        params = {
            "trCode": "OPT10074",
            "계좌번호": self.accNo,
            "시작일자": self.date,
            "종료일자": self.date,
        }

        response = self.client.get(self.url, data=params)
        data = response.json()

        self.assertEqual(response.status_code, 200)
        self.assertGreater(len(data), 0)

    def test_OPT10074_bad_accno(self):
        """ OPT10074 계좌번호가 틀린 경우 """
        params = {
            "trCode": "OPT10074",
            "계좌번호": "fbgshbfgqwenjnfjgknsdfkgnsuidfygvbsdf",
            "시작일자": self.date,
            "종료일자": self.date,
        }

        response = self.client.get(self.url, data=params)
        data = response.content.decode("utf-8")

        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, "Bad Request")

    #!TODO: 날짜가 틀린 경우, Kiwoom에서 에러 발생시키도록 수정?
    def test_OPT10074_bad_date(self):
        """ OPT10074 날짜가 틀린 경우 """
        params = {
            "trCode": "OPT10074",
            "계좌번호": self.accNo,
            "시작일자": "99999999",
            "종료일자": "99999999",
        }

        response = self.client.get(self.url, data=params)
        data = response.content.decode("utf-8")
        print(data)
        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, "Bad Request")

