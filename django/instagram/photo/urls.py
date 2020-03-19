from django.urls import path, re_path
from .views import *

app_name = "photo"
urlpatterns = [
    path("", PhotoList.as_view(), name="index"),  # CBV는 as_view(cls_method) 호출
    path("create/", PhotoCreate.as_view(), name="create"),
    path("update/<int:pk>/", PhotoUpdate.as_view(), name="update"),
    path("delete/<int:pk>/", PhotoDelete.as_view(), name="delete"),
    path("detail/<int:pk>/", PhotoDetail.as_view(), name="detail"),
    path("like/<int:pk>/", PhotoLike.as_view(), name="like"),
    path("favorite/<int:pk>", PhotoFavorite.as_view(), name="favorite"),
    path("my-likes", PhotoLikeList.as_view(), name="my_likes"),
]
