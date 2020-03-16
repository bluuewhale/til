from django.urls import path, re_path, include
from polls import views

app_name = "polls"

urlpatterns = [
    path("", views.index, name="index"),
    path("<int:id>/", views.detail, name="detail"),
    path("<int:id>/results/", views.results, name="results"),
    path("<int:id>/vote/", views.vote, name="vote"),
]
