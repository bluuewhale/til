from django.urls import path, re_path, include
from tr_request import views


urlpatterns = [
    re_path('^', views.tr_request)
]
    