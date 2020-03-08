from django.conf.urls import url, include
from rest_framework.routers import DefaultRouter

from api.views import *

router = DefaultRouter()

urlpatterns = [
    # url(r"^(?P<ticker>[a-zA-Z0-9]+)", OHLCAPIView.as_view()),
    url(r"^ticker=(?P<ticker>[a-zA-Z0-9]+)", get_ohlc, name="ohlc"),
    # url(r"^ticker=(?P<ticker>[a-zA-Z0-9]+$)", get_ticker, name="get_ticker"),
]
