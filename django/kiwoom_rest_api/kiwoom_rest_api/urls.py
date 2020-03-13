from django.contrib import admin
from django.urls import path, re_path, include

urlpatterns = [
    path('admin/', admin.site.urls),
    path('health-check/', include('health_check.urls')),
    path('data/', include('data_request.urls')),
]
