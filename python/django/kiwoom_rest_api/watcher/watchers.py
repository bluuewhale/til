import asyncio
from datetime import datetime
import os
import signal
import sys

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.conf import settings

settings.configure()

from decorators import AutoRunDecorator
from kiwoom_api.api import Kiwoom, DataFeeder
from kiwoom_api.utility.utility import readTxt, saveTxt
from utility import read_json, write_json


class KiwoomWatcher:
    def __init__(self, kiwoom, worker, *args, **kwargs):
        self.kiwoom = kiwoom
        self.worker = worker

        for k, v in kwargs.items():
            setattr(self, k, v)

        self.clear()

    @AutoRunDecorator.asyncFullTime(delay=0.05)
    async def run(self, main_func_name):
        reqs = os.listdir(self.req_dir)

        if reqs:
            req_name = reqs[0]
            req_path = os.path.join(self.req_dir, req_name)
            req_content = read_json(req_path)
            os.remove(req_path)  # 읽은 후 바로 삭제

            res = getattr(self.worker, main_func_name)(**req_content)
            res_path = os.path.join(self.res_dir, req_name)
            write_json(res, res_path)  # res 파일 생성

    @property
    def req_dir(self):
        raise NotImplementedError()

    @property
    def res_dir(self):
        raise NotImplementedError()

    def clear(self):
        """ clear all requests and responses """
        for f in os.listdir(self.req_dir):
            f_path = os.path.join(self.req_dir, f)
            os.remove(f_path)

        for f in os.listdir(self.res_dir):
            f_path = os.path.join(self.res_dir, f)
            os.remove(f_path)


class TrWatcher(KiwoomWatcher):
    def __init__(self, kiwoom, worker, *args, **kwargs):
        super().__init__(kiwoom, worker, *args, **kwargs)

    @property
    def req_dir(self):
        return "cache/tr/requests"

    @property
    def res_dir(self):
        return "cache/tr/responses"


class OrderWatcher(KiwoomWatcher):
    def __init__(self, kiwoom, worker, *args, **kwargs):
        super().__init__(kiwoom, worker, *args, **kwargs)

    @property
    def req_dir(self):
        return "cache/order/requests"

    @property
    def res_dir(self):
        return "cache/order/responses"

