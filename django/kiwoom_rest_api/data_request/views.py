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
    def now(self):
        return dt.now().strftime("%Y-%m-%d-%H-%M-%S.%f")
    
    @property
    def params(self):
        return {k: v for k, v in getattr(self.request, 'GET').items()}

    @property
    def content(self):
        content_dict = self.__dict__.copy()
        content_dict.pop('request')
        content_dict.update(self.params)
        return content_dict


def request(request, trCode):
    
    request = Request(request, trCode=trCode)
    content = request.content

    base_path = '{}/{}.json'
    req_folder = os.path.join(settings.BASE_DIR, 'watcher', 'data_requests')
    file_name = request.now
    req_path = base_path.format(req_folder, file_name)
    write_json(content, req_path) # req 파일 생성

    for _ in range(100):
        time.sleep(0.01)
        try:
            res_folder = os.path.join(settings.BASE_DIR, 'watcher', 'data_responses')
            res_path = base_path.format(res_folder, file_name)
            data = read_json(res_path)
            os.remove(res_path) # res 파일 삭제
            return JsonResponse(data)
        except FileNotFoundError:
            pass
    
    return {}