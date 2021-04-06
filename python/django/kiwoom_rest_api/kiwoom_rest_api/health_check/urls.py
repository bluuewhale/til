from django.urls import path, re_path, include
from health_check import views

urlpatterns = [
    path('connection', views.connection_state)
]
