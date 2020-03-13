from django.conf.urls import url, include
from django.contrib import admin
from rest_framework import routers

from movies.api.views import *

# 라우터를 생성하고 ViewSet을 등록
router = routers.DefaultRouter()
router.register("", MovieViewSet)

# 이제는 API URL을 라우터가 자동으로 인식

urlpatterns = [
    # -------------------------------------
    # 클래스형 View
    # -------------------------------------
    # url(r"^$", MovieListAPIView.as_view(), name="movie_list"),
    # url(r"^(?P<id>\d+)$", MovieDetailAPIView.as_view(), name="movie_detail"),
    # url(r"^update/(?P<id>\d+)$", MovieUpdateAPIView.as_view(), name="movie_update"),
    # url(r"^delete/(?P<id>\d+)$", MovieDestroyAPIView.as_view(), name="movie_delete"),
    # url(r"^create$", MovieCreateAPIView.as_view(), name="movie_create"),
    # -------------------------------------
    # Class ViewSet
    # -------------------------------------
    url(r"^", include(router.urls)),
    # -------------------------------------
    # 함수형 View (FBV, Function Based View)
    # -------------------------------------
    # url(r"^$", index, name="movie_list"),
]
