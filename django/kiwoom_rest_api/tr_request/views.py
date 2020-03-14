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

class Request:
    def __init__(self, request, *args, **kwargs):
        self.request = request

        for k, v in kwargs.items():
            setattr(self, k, v)

    @property
    def params(self):
        return {k: v for k, v in getattr(self.request, 'GET').items()}

    def get_content(self, **kwargs):
        content_dict = {k: v for k, v in kwargs.items()}
        content_dict.update(self.params)
        return content_dict
    
class Response:
    def __init__(self):
        pass 

    def read_json(self, path, n_retry=100, delay=0.1):
        for _ in range(n_retry):
            time.sleep(delay)
            try:
                data = read_json(path)
                os.remove(path) # res 파일 삭제
                return JsonResponse(data)
            except FileNotFoundError:
                pass
        return JsonResponse({})

class TrManager(Request, Response):
    def __init__(self, request, req_dir, res_dir, *args, **kwargs):
        #super(TrManager, self).__init__(self, request, *args, **kwargs) # init Request only
        self.request = request
        self.req_dir = req_dir
        self.res_dir = res_dir
        
        self.name = self.now
        self.content = self.get_content(**kwargs)

        for k, v in kwargs.items():
            setattr(self, k, v)
    
    @property
    def now(self):
        return dt.now().strftime("%Y-%m-%d-%H-%M-%S.%f")
    
    def __send_request(self):
        path = os.path.join(self.req_dir, self.name)
        write_json(self.content, path)
    
    def __get_response(self, n_retry, delay):
        path = os.path.join(self.res_dir, self.name)
        return self.read_json(path, n_retry, delay)
    
    def run(self, n_retry=30, delay=0.1):
        
        self.__send_request()
        return self.__get_response(n_retry, delay)

def tr_request(request, trCode):
    
    req_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_requests')
    res_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_responses')
    manager = TrManager(request, req_dir, res_dir, trCode=trCode)
    data =  manager.run()
    return data