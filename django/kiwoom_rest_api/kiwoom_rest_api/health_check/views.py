from django.shortcuts import render
from django.http import HttpResponse, JsonResponse

from django.conf import settings 

def connection_state(request):
    return HttpResponse(settings.KIWOOM.connectState)
