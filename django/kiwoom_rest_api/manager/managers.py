from datetime import datetime as dt
import os
import sys
import time

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.http import JsonResponse
from django.conf import settings

from utility import write_json, read_json


class Manager:

    def __init__(self, req_dir, res_dir):
        file_name = dt.now().strftime("%Y%m%d%H%M%S.%f") + '.json'  # request 중복 방지를 위해 현재 시간을 사용
        self.req_path = os.path.join(req_dir, file_name)
        self.res_path = os.path.join(res_dir, file_name)

    def run(self, request):
        raise NotImplementedError()

    @staticmethod
    def parse_params(request, method='GET'):
        return {k: v for k, v in getattr(request, method).items()}

    def _send_request(self, content):
        write_json(content, self.req_path)

    def _get_response(self):
        raise NotImplementedError()

class TrManager(Manager):
    
    def __init__(self, req_dir, res_dir, *args, **kwargs):
        super().__init__(req_dir, res_dir)
        
        for k, v in kwargs.items():
            setattr(self, k, v)
    
    def run(self, request, n_retry=30, delay=0.1):

        content = self.parse_params(request)
        self._send_request(content)
        return self._get_response(n_retry, delay)

    def _get_response(self, n_retry, delay):
       
        for _ in range(n_retry):
            time.sleep(delay)
            try:
                data = read_json(self.res_path)
                os.remove(self.res_path) # res 파일 삭제
                return JsonResponse(data)
                
            except FileNotFoundError:
                pass

        return JsonResponse({'status':400}, status=400)
    

class OrderManager(Manager):
    def __init__(self, req_dir, res_dir, *args, **kwargs):
        super().__init__(req_dir, res_dir)
        
        for k, v in kwargs.items():
            setattr(self, k, v)
    
    def _get_response(self, n_retry, delay):
        for _ in range(n_retry):
            time.sleep(delay)
            try:
                data = read_json(self.res_path)
                os.remove(self.res_path) # res 파일 삭제
                return JsonResponse(data)
                
            except FileNotFoundError:
                pass

        return JsonResponse({'status':400}, status=400)
    
    def run(self, request):
        content = self.parse_params(request)
        self._send_request(content)
        return self._get_response()

