import json
from pprint import pprint
import requests
import unittest


class OrderTest(unittest.TestCase):
    def setUp(self):
        self.url = "http://127.0.0.1:8000/order/"
        self.code = "005930"
        self.accNo = "8131214911"
        self.token = "Token d7efae0bbe54e6d994de5b9b83474abb5330ce4f"

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

        response = requests.post(self.url, data=params, headers=headers)
        try:
            data = json.loads(response.content)
            pprint(data)
        except:
            print(response.content)


if __name__ == "__main__":
    unittest.main()
