from django.urls import path, re_path, include
from order_request import views

app_name = "order_request"

urlpatterns = [
    re_path("", views.OrderRequestView.as_view(), name="request"),
]

