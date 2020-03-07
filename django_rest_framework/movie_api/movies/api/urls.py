from django.conf.urls import url
from django.contrib import admin

from movies.api.views import *


urlpatterns = [
    # 클래스형 View
    # url(r"^$", MovieListAPIView.as_view(), name="movie_list"),
    # url(r"^(?P<id>\d+)$", MovieDetailAPIView.as_view(), name="movie_detail"),
    # url(r"^update/(?P<id>\d+)$", MovieUpdateAPIView.as_view(), name="movie_update"),
    # url(r"^delete/(?P<id>\d+)$", MovieDestroyAPIView.as_view(), name="movie_delete"),
    # url(r"^create$", MovieCreateAPIView.as_view(), name="movie_create"),
    # 함수형 View (FBV, Function Based View)
    url(r"^$", index, name="movie_list"),
]
