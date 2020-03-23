import json
from datetime import datetime as dt
import os
import sys
import time

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.conf import settings
from django.http import JsonResponse, HttpResponse
from rest_framework.authtoken.models import Token
from rest_framework.decorators import api_view, permission_classes
from rest_framework.permissions import IsAuthenticated

from manager import OrderManager
from .models import Order


@api_view(["POST"])
@permission_classes((IsAuthenticated,))
def order_request(request):
    req_dir = os.path.join(settings.BASE_DIR, "watcher/cache/order/requests")
    res_dir = os.path.join(settings.BASE_DIR, "watcher/cache/order/responses")

    manager = OrderManager(request, req_dir, res_dir)
    data = manager.run()

    """ save order log """
    token = request.META.get("HTTP_AUTHORIZATION").split(" ")[-1]
    user = getattr(Token.objects.get(key=token), "user")
    order = Order.objects.create(user=user, **data)

    if data is None:
        return HttpResponse("Bad Request", status=400)
    return JsonResponse(data)
