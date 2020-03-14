import json
import os
import requests
from urllib.parse import urlencode 
import unittest
#from django.test import TestCase

# Create your tests here.
class RequestTest(unittest.TestCase):
    
    def setUp(self):
        self.url = 'http://127.0.0.1:8000/data/'
    
    def test_OTP10004(self):

        url = os.path.join(self.url, 'OPT10005')
        params = {'종목코드' : '005930'}
        
        response = requests.get(url, params=params)
        #print(response.content)
        print(json.loads(response.content))


if __name__ == "__main__":
    unittest.main()


