from django.urls import path, re_path, include
from data_request import views


urlpatterns = [
    re_path('^(?P<trCode>[0-9a-zA-Z]+)/', views.request)
]
    