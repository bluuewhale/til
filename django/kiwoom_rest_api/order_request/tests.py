import json
import requests
import unittest

class OrderTest(unittest.TestCase):
    
    def setUp(self):
        self.url = 'http://127.0.0.1:8000/order/'
        self.code = "005sdfsfd930"
        self.accNo = "8131214911"

    def test_buy_order(self):
        params = {
            "rqName":"test",
            "scrNo":"0000",
            "accNo":self.accNo,
            "orderType":1,  # 신규매수
            "code":self.code,
            "qty":1,
            "price":0,
            "hogaType":"03",
            "originOrderNo":"",
        }

        response = requests.get(self.url, params)
        #data = json.loads(response.content)
        print(response.content)

if __name__ == "__main__":
    unittest.main()
