import json
from datetime import datetime as dt
import os
import sys
import time

sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from django.conf import settings
from django.http import JsonResponse, HttpResponse
from rest_framework.authtoken.models import Token
from rest_framework.permissions import IsAuthenticated
from rest_framework.views import APIView

from manager import OrderManager
from .models import Order


class OrderRequestView(APIView):
    permission_classes = [IsAuthenticated]

    def post(self, request, format=None):
        req_dir = os.path.join(settings.BASE_DIR, "watcher/cache/order/requests")
        res_dir = os.path.join(settings.BASE_DIR, "watcher/cache/order/responses")

        manager = OrderManager(request, req_dir, res_dir)
        data = manager.run()
        if data is None:
            return HttpResponse("Bad Request", status=400)

        """ save order log """
        token = request.META.get("HTTP_AUTHORIZATION").split(" ")[-1]
        user = getattr(Token.objects.get(key=token), "user")
        order = Order.objects.create(user=user, **data)

        return JsonResponse(data)
