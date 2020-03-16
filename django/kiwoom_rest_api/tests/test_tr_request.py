import json
import os
from pprint import pprint
import requests
import sys
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from urllib.parse import urlencode 
import unittest


from utility import read_txt

class RequestTest(unittest.TestCase):
    
    def setUp(self):

        self.url = 'http://127.0.0.1:8000/tr/'
        token = read_txt('token.txt')
        self.headers = {'Authorization': f'Token {token}'}

        self.code = '005930'
        self.date = '20200314'
        self.accNo = '8131214911'
        
    def test_OTP10004(self):

        params = {
            'trCode':'OPT10004', 
            '종목코드' : self.code,
        }
        
        response = requests.get(self.url, headers=self.headers, params=params)
        data = response.json()
        
        self.assertGreater(len(data), 0)
        #pprint(data)

    def test_OTP10004_bad_trCode(self):

        params = {
            'trCode':'OPT10004aaaaaa', 
            '종목코드' : self.code,
        }
        
        response = requests.get(self.url, headers=self.headers, params=params)
        data = response.content
        
        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, b'Bad Request')

    def test_OTP10004_bad_code(self):

        params = {
            'trCode':'OPT10004', 
            '종목코드' : 'sdfgnsfdgsndfig',
        }
        
        response = requests.get(self.url, headers=self.headers, params=params)
        data = response.content
        
        self.assertEqual(response.status_code, 400)
        self.assertEqual(data, b'Bad Request')

    def test_OPT10005(self):

        params = {
            'trCode' : 'OPT10005',
            "종목코드": self.code
        }
        
        response = requests.get(self.url, headers=self.headers, params=params)
        #pprint(response.json())


    def test_OPT10059(self):

        params = {
            'trCode' : 'OPT10059',
            "일자": self.date,
            "종목코드": self.code,
            "금액수량구분": "1",  # 1:금액, 2:수량
            "매매구분": "0",  # 0:순매수, 1:매수, 2:매도
            "단위구분": "1",  # 1:단주, 1000:천주
        }

        response = requests.get(self.url, headers=self.headers, params=params)
        #pprint(response.json())

    def test_OPT10074(self):

        params = {
            'trCode' : 'OPT10074',
            "계좌번호": self.accNo,
            "시작일자": self.date,
            "종료일자": self.date,
        }

        response = requests.get(self.url, headers=self.headers, params=params)
        #pprint(response.json())

    def test_OPT10074_bad_acc(self):

        params = {
            'trCode' : 'OPT10074',
            "계좌번호": 'fbgshbfgqwenjnfjgknsdfkgnsuidfygvbsdf',
            "시작일자": self.date,
            "종료일자": self.date,
        }

        response = requests.get(self.url, headers=self.headers, params=params)
        self.assertEqual(response.status_code, 400)

if __name__ == "__main__":
    unittest.main()
