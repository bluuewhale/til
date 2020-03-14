import json
from datetime import datetime as dt
import os
import sys
import time
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.conf import settings 

from manager import TrManager

def tr_request(request):
    req_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_requests', )
    res_dir = os.path.join(settings.BASE_DIR, 'watcher/tr_responses')
    
    manager = TrManager(req_dir, res_dir)
    data =  manager.run(request)
    return data