import json
from datetime import datetime as dt
import os
import sys
import time

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.conf import settings
from rest_framework.decorators import api_view, permission_classes
from rest_framework.permissions import IsAuthenticated
from manager import TrManager


@api_view(["GET"])
@permission_classes((IsAuthenticated,))
def tr_request(request):
    req_dir = os.path.join(settings.BASE_DIR, "watcher/cache/tr/requests")
    res_dir = os.path.join(settings.BASE_DIR, "watcher/cache/tr/responses")

    manager = TrManager(request, req_dir, res_dir)
    data = manager.run()
    return data
