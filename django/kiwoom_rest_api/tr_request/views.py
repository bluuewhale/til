import json
from datetime import datetime as dt
import os
import sys
import time
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.shortcuts import render
from django.http import HttpResponse, JsonResponse
from django.conf import settings 

from utility import write_json, read_json

def parse_params(request, method='GET'):
    return {k: v for k, v in getattr(request, method).items()}

class TrManager:
    def __init__(self, req_dir, res_dir, *args, **kwargs):
        self.req_dir = req_dir
        self.res_dir = res_dir
        self.name = self.now

        for k, v in kwargs.items():
            setattr(self, k, v)
    
    @property
    def now(self):
        return dt.now().strftime("%Y-%m-%d-%H-%M-%S.%f")
    
    def __send_request(self, content):
        path = os.path.join(self.req_dir, self.name)
        write_json(content, path)
    
    def __get_response(self, n_retry, delay):
        path = os.path.join(self.res_dir, self.name)
        
        for _ in range(n_retry):
            time.sleep(delay)
            try:
                data = read_json(path)
                os.remove(path) # res 파일 삭제
                return JsonResponse(data)
            except FileNotFoundError:
                pass

        return JsonResponse({})
    
    def run(self, content, n_retry=30, delay=0.1):
        self.__send_request(content)
        return self.__get_response(n_retry, delay)

def tr_request(request):
    
    req_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_requests')
    res_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_responses')
    manager = TrManager(req_dir, res_dir)

    req_contents = parse_params(request)
    data =  manager.run(req_contents)
    return data