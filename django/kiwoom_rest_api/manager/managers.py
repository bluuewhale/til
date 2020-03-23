from datetime import datetime as dt
import io
import os
import sys
import time

# sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from rest_framework.parsers import JSONParser

from utility import write_json, read_json
from order_request.serializers import OrderSerializer


class Manager:
    def __init__(self, request, req_dir, res_dir, *args, **kwargs):

        self.request = request
        file_name = dt.now().strftime("%Y%m%d%H%M%S.%f") + ".json"
        self.req_path = os.path.join(req_dir, file_name)
        self.res_path = os.path.join(res_dir, file_name)

        for k, v in kwargs.items():
            setattr(self, k, v)

    def run(self, n_retry=40, delay=0.05):

        content = self.parse_params(self.request)
        self._send_request(content)
        return self._get_response(n_retry, delay)  # json

    @staticmethod
    def parse_params(request):
        method = getattr(request, "method")
        return {k: v for k, v in getattr(request, method).items()}

    def _send_request(self, content):
        write_json(content, self.req_path)

    def _get_response(self, n_retry, delay):

        for _ in range(n_retry):
            time.sleep(delay)
            try:
                data = read_json(self.res_path)
                os.remove(self.res_path)  # res 파일 삭제
                return data
            except FileNotFoundError:
                pass
        return None


class TrManager(Manager):
    def __init__(self, req_dir, res_dir, *args, **kwargs):
        super().__init__(req_dir, res_dir, *args, **kwargs)


class OrderManager(Manager):
    def __init__(self, req_dir, res_dir, *args, **kwargs):
        self.serializer_cls = OrderSerializer
        super().__init__(req_dir, res_dir, *args, **kwargs)
