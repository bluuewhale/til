from django.urls import path, re_path, include
from tr_request import views

app_name = "tr_request"

urlpatterns = [re_path("", views.OrderRequestView.as_view(), name="request")]

