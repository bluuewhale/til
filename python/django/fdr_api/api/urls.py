from django.conf.urls import url
from api.views import *

urlpatterns = [
    url(r"^$", get_ohlc, name="ohlc"),
]
