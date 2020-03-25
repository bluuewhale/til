from django.urls import path, re_path, include
from tr_request import views

app_name = "tr_request"

urlpatterns = [
    re_path(
        "(?P<trCode>[a-zA-Z0-9]{8})", views.OrderRequestView.as_view(), name="request",
    ),
    path("", views.OrderBaseView, name="base"),
]
