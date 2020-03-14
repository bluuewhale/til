from django.urls import path, re_path, include
from tr_request import views


urlpatterns = [
    re_path('^(?P<trCode>[0-9a-zA-Z]+)/', views.tr_request)
]
    