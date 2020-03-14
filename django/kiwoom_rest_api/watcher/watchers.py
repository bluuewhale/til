import asyncio
from datetime import datetime
import os
import signal
import sys
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

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
    
    @property
    def req_dir(self):
        raise NotImplementedError()

    @property
    def res_dir(self):
        raise NotImplementedError()
    
    @AutoRunDecorator.asyncFullTime(delay=0.05)
    async def asyncRun(self):
        raise NotImplementedError

    # Support Methods
    def __killOldProcess(self):
        # 이전에 실행된 Process는 꺼버리고 새로운 Process로 실행 (중복 방지).
        try:
            pidPath = 'pid.txt'
            pid = int(readTxt(pidPath))
            os.kill(pid, signal.SIGTERM)
        except:
            pass
        finally: # 새롭게 실행한 Process ID를 기록
            saveTxt(os.getpid(), pidPath)
            

class TrWatcher(KiwoomWatcher):
    def __init__(self, kiwoom, worker, *args, **kwargs):
        super().__init__(kiwoom, worker, *args, **kwargs)

    @AutoRunDecorator.asyncFullTime(delay=0.05)
    async def asyncRun(self):
        reqs = os.listdir(self.req_dir)
        
        if reqs:
            req_name = reqs[0]
            req_path = os.path.join(self.req_dir, req_name)
            req_content = read_json(req_path)
            os.remove(req_path) # 읽은 후 바로 삭제
            
            res = self.worker.request(**req_content) # request 전송, response 수신
            res_path = os.path.join(self.res_dir, req_name)
            write_json(res, res_path) # res 파일 생성

    @property
    def req_dir(self):
        return 'tr_requests'

    @property
    def res_dir(self):
        return 'tr_responses'


class OrderWatcher(KiwoomWatcher):
    def __init__(self, kiwoom, worker, *args, **kwargs):
        super().__init__(kiwoom, worker, *args, **kwargs)

    @AutoRunDecorator.asyncFullTime(delay=0.05)
    async def asyncRun(self):
        reqs = os.listdir(self.req_dir)
        
        if reqs:
            req_name = reqs[0]
            req_path = os.path.join(self.req_dir, req_name)
            req_content = read_json(req_path)
            os.remove(req_path) # 읽은 후 바로 삭제
            
            res = self.worker.sendOrder(**req_content) # order 전송
            res_path = os.path.join(self.res_dir, req_name)
            write_json(res, res_path) # res 파일 생성

    @property
    def req_dir(self):
        return 'order_requests'

    @property
    def res_dir(self):
        return 'order_responses'
    