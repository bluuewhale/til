from django.urls import path
from django.contrib.auth.views import LogoutView
from .views import *

app_name = "accounts"

urlpatterns = [
    path("login/", UserLoginView.as_view(), name="login"),
    path(
        "logout/",
        LogoutView.as_view(template_name="accounts/logout.html"),
        name="logout",
    ),
    path("signup/", SignUpView.as_view(), name="signup"),
]

