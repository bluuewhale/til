from django.urls import path, re_path, include
from order_request import views


urlpatterns = [
    re_path('', views.order_request)
]
    